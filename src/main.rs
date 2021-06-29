#![feature(test)]
//use std::fmt::Debug;

fn swap<'a, T: Copy /*+Debug*/>(array: &'a mut [T], a: usize, b: usize) {
    //println!("swapping {} with {} in array {:?}", a, b, array);
    let tmp = array[a];
    array[a] = array[b];
    array[b] = tmp;
}

fn sort_one_1<'a, T: Copy + PartialOrd>(v: &'a mut [T], low: usize, high: usize) -> usize {
    let mut l = low;
    let mut h = high;
    let mid = v[l];
    loop {
        while l < h && mid <= v[h] {
            h -= 1;
        }
        if l == h {
            break;
        }
        v[l] = v[h];
        l += 1;
        while l < h && v[l] <= mid {
            l += 1;
        }
        if l == h {
            break;
        }
        v[h] = v[l];
        h -= 1;
    }
    v[l] = mid;
    return l;
}

fn sort_one_2<'a, T: Copy + PartialOrd /*+Debug*/>(
    v: &'a mut [T],
    low: usize,
    high: usize,
) -> usize {
    let mid = v[high];
    let mut i = low;
    let mut j = low;
    while j <= high {
        if v[j] < mid {
            i += 1;
            swap(v, i - 1, j);
        }
        j += 1;
    }
    swap(v, i, high);
    return i;
}

fn add_intervals<'a, 'b>(stack: &'a mut Vec<Interval>, middle: usize, last: &'b Interval) {
    if middle > 0 && last.low < middle - 1 {
        let i = Interval {
            low: last.low,
            high: middle - 1,
        };
        stack.push(i);
    }
    if last.high > middle + 1 {
        let i = Interval {
            low: middle + 1,
            high: last.high,
        };
        stack.push(i);
    }
}

#[derive(Debug)]
struct Interval {
    low: usize,
    high: usize,
}
fn __sort<'a, T>(v: &'a mut [T], sort_one_func: fn(&mut [T], usize, usize) -> usize) {
    let mut intervals = Vec::new();
    intervals.push(Interval {
        low: 0,
        high: v.len() - 1,
    });
    let mut max = v.len();
    loop {
        max -= 1;
        if max == 0 {
            return;
        }
        match intervals.pop() {
            Some(i) => {
                //println!("sorting interval [{}, {}]", i.low, i.high);
                let mid = sort_one_func(v, i.low, i.high);
                add_intervals(&mut intervals, mid, &i);
            }
            None => return,
        }
    }
}
fn sort_1<'a, T: Copy + PartialOrd>(v: &'a mut [T]) {
    return __sort(v, sort_one_1);
}
fn sort_2<'a, T: Copy + PartialOrd /*+Debug*/>(v: &'a mut [T]) {
    return __sort(v, sort_one_2);
}

#[cfg(test)]
mod tests {
    macro_rules! sort_one_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = $value;
                let mut result_1 = input;
                let mid = crate::sort_one_1(&mut result_1, 0, input.len() - 1);
                for i in 0 ..= mid {
                    for j in mid .. input.len() {
                        assert!(result_1[i] <= result_1[j], "\nexpected: {:?} [{}] <= [{}]\nmid={}", result_1, i, j, mid);
                    }
                }
                let mut result_2 = input;
                let mid = crate::sort_one_2(&mut result_2, 0, input.len() - 1);
                for i in 0 ..= mid {
                    for j in mid .. input.len() {
                        assert!(result_2[i] <= result_2[j], "\nexpected: {:?} [{}] <= [{}]\nmid={}", result_2, i, j, mid);
                    }
                }
            }
        )*
        }
    }
    sort_one_tests! {
        sort_one_upper_half: [7, 7, 8, 34, 1],
    }

    macro_rules! sort_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let mut result = input;
                crate::sort_1(&mut result);
                assert_eq!(expected, result, "\n input: `{:?}`", input);
            }
        )*
        }
    }
    sort_tests! {
        sort_simple_2: ([1, 0], [0, 1]),
        sort_simple_4: ([2, 2, 1, 1], [1, 1, 2, 2]),
        sort_simple_6: ([3, 3, 2, 2, 1, 1], [1, 1, 2, 2, 3, 3]),
        sort_intermixed_6: ([3, 1, 2, 3, 2, 1], [1, 1, 2, 2, 3, 3]),
        sort_upper_half: ([7, 7, 8, 34, 1], [1, 7, 7, 8, 34]),
    }

    #[test]
    fn fuzzing() {
        let mut rand = Rand::new(0x1337);
        for _ in 0..10000 {
            let mut v = random_vector(
                &mut rand,
                &crate::Interval { low: 2, high: 30 },
                &crate::Interval { low: 0, high: 5 },
            );
            let copy = v.clone();
            crate::sort_1(v.as_mut_slice());
            println!(
                "len={},\n  unsorted={:?},\n  sorted  ={:?}",
                v.len(),
                copy,
                v
            );
            for n in 0..v.len() - 2 {
                assert!(
                    v[n as usize] <= v[(n + 1) as usize],
                    "\nexpected index [{}]={} <= [{}]={}",
                    n,
                    v[n as usize],
                    n + 1,
                    v[(n + 1) as usize]
                );
            }
        }
    }
    struct Rand {
        state: u32,
    }
    // linear congruential method
    impl Rand {
        const A: u32 = 5u32; // must be % 4 == 1 and < u32::MAX
        const C: u32 = 3u32; // must be odd      and < u32::MAX
        fn new(seed: u32) -> Rand {
            return Rand { state: seed };
        }
        fn gen(&mut self, i: &crate::Interval) -> u32 {
            self.state = self.state.wrapping_mul(Rand::A).wrapping_add(Rand::C);
            return (self.state % (i.high - i.low) as u32) + i.low as u32;
        }
    }

    fn random_vector<'a, 'b>(
        rnd: &'a mut Rand,
        len_range: &'b crate::Interval,
        num_range: &'b crate::Interval,
    ) -> Vec<i32> {
        let len = rnd.gen(len_range);
        let mut vec = Vec::new();
        for _ in 0..len {
            vec.push(rnd.gen(num_range) as i32);
        }
        return vec;
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_1(b: &mut Bencher) {
        let mut rand = Rand::new(0x1337);
        let mut vec = random_vector(
            &mut rand,
            &crate::Interval {
                low: 10000,
                high: 100000,
            },
            &crate::Interval {
                low: 0,
                high: 100000,
            },
        );
        b.iter(|| crate::sort_1(&mut vec));
    }

    #[bench]
    fn bench_2(b: &mut Bencher) {
        let mut rand = Rand::new(0x1337);
        let mut vec = random_vector(
            &mut rand,
            &crate::Interval {
                low: 10000,
                high: 100000,
            },
            &crate::Interval {
                low: 0,
                high: 100000,
            },
        );
        b.iter(|| crate::sort_2(&mut vec));
    }
}

fn main() {}
