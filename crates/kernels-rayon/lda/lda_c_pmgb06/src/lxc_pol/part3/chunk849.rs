//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 849/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk849(t348: f64, t350: f64, t3611: f64, t3500: f64, t3510: f64, t61: f64, t8337: f64, t3616: f64, t365: f64, t1179: f64, t342: f64, t55: f64) -> (f64, f64, f64, f64) {
    let t8341 = t348 * t3611 * t350;
    let t8346 = 0.16322666666666666_f64 * t61 * t3500 * t3510 * t8337;
    let t8348 = t365 * t3616 * t350;
    let t8352 = t55 * t1179 * t342;
    (t8341, t8346, t8348, t8352)
}
