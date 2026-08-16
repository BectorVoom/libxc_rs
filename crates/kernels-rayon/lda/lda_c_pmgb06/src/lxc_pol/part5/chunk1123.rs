//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1123/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1123(t16869: f64, t16875: f64, t2090: f64, t2563: f64, t1848: f64, t2654: f64, t6461: f64, t831: f64, t20478: f64, t20480: f64, t20482: f64, t20486: f64, t20490: f64, t20491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20492 = t16869 / 15.0_f64;
    let t20493 = 2.0_f64 / 15.0_f64 * t16875;
    let t20495 = t2563 * t2090 / 10.0_f64;
    let t20497 = t1848 * t2654 / 5.0_f64;
    let t20499 = t831 * t6461 / 5.0_f64;
    let t20500 = t20478 - t20480 + t20482 - t20486 + t20490 - t20491 - t20492 - t20493 - t20495 - t20497 - t20499;
    (t20492, t20493, t20495, t20497, t20499, t20500)
}
