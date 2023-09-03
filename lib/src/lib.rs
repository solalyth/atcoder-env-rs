mod print; pub use print::*;
mod solver; pub use solver::*;

mod modulo; pub use modulo::*;
mod bit; pub use bit::*;
mod nest; pub use nest::*;
mod others; pub use others::*;

// for debug
pub use proconio::source::once::OnceSource;

pub use {
    proconio::{input, marker::{Chars, Usize1}},
    std::cmp::{min, max, Reverse as Rev},
    std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    itertools::Itertools,
    superslice::Ext
};