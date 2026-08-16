//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 661/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk661(t1271: f64, t370: f64, t97: f64, t315: f64, t342: f64, t934: f64, t109: f64, t1227: f64, t55: f64, t1276: f64, t1238: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3566 = t1271 * t370 * t97;
    let t3568 = t934 * t315 * t342;
    let t3569 = t3566 * t3568;
    let t3572 = t55 * t109 * t1227;
    let t3573 = t1276 * t3572;
    let t3576 = t1238 * t56 * t97;
    (t3566, t3568, t3569, t3572, t3573, t3576)
}
