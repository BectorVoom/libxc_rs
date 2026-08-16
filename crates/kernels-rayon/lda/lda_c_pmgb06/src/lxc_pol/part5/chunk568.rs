//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 568/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk568(t1272: f64, t2060: f64, t1239: f64, t361: f64, t410: f64, t360: f64, t127: f64, t9: f64, t14: f64, t158: f64, t1271: f64, t370: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3523 = 0.6529066666666666_f64 * t1272 * t2060;
    let t3525 = 1.2991222222222223_f64 * t1239 * t2060;
    let t3530 = t410 * t361;
    let t3531 = t360 * t3530;
    let t3537 = 1.0_f64 / t9 / t127;
    let t3548 = 1.0_f64 / t14 / t158;
    let t3566 = t1271 * t370 * t97;
    (t3523, t3525, t3530, t3531, t3537, t3548, t3566)
}
