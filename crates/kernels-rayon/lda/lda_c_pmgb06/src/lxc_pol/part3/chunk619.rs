//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 619/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk619(t1239: f64, t2060: f64, t1227: f64, t371: f64, t361: f64, t410: f64, t360: f64, t110: f64, t1267: f64, t127: f64, t9: f64, t1212: f64, t332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3525 = 1.2991222222222223_f64 * t1239 * t2060;
    let t3526 = t371 * t1227;
    let t3530 = t410 * t361;
    let t3531 = t360 * t3530;
    let t3533 = t110 * t1267;
    let t3534 = t360 * t3533;
    let t3537 = 1.0_f64 / t9 / t127;
    let t3540 = t1212 * t332;
    (t3525, t3526, t3530, t3531, t3533, t3534, t3537, t3540)
}
