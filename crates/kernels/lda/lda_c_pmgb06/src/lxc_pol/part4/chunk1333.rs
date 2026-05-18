//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1333/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1333<F: Float>(t1602: F, t1992: F, t2599: F, t493: F, t9636: F, t439: F, t5197: F, t6258: F, t1444: F, t6287: F, t6528: F, t6254: F) -> (F, F, F, F, F) {
    let t17527 = F::new(4.0) / F::new(5.0) * t493 * t1992 * t9636 * t2599 * t1602;
    let t17530 = F::new(4.0) / F::new(15.0) * t439 * t5197 * t6258;
    let t17532 = F::new(2.0) / F::new(5.0) * t1444 * t6287;
    let t17534 = F::new(4.0) / F::new(15.0) * t1444 * t6528;
    let t17537 = F::new(2.0) / F::new(5.0) * t439 * t5197 * t6254;
    (t17527, t17530, t17532, t17534, t17537)
}
