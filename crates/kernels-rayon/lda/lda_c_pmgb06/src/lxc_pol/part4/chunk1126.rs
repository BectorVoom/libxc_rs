//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1126/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1126(t415: f64, t5522: f64, t2249: f64, t384: f64, t387: f64, t5887: f64, t707: f64, t5891: f64, t5895: f64, t1770: f64, t419: f64, t4238: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14549 = t5522 * t415;
    let t14561 = t387 * t384 * t2249;
    let t14567 = t707 * t5887;
    let t14569 = t707 * t5891;
    let t14571 = t707 * t5895;
    let t14575 = t4238 * t794 * t419 * t1770;
    (t14549, t14561, t14567, t14569, t14571, t14575)
}
