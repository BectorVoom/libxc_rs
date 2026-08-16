//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 974/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk974(t20: f64, t369: f64, t3501: f64, t3502: f64, t642: f64, t3509: f64, t3510: f64, t56: f64, t247: f64, t28: f64, t342: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8245 = 1.0_f64 / t369 / t20;
    let t8263 = 15.589466666666667_f64 * t3501 * t3502 * t642;
    let t8266 = 2.6116266666666665_f64 * t3509 * t3510 * t642;
    let t8276 = t3501 * t56;
    let t8278 = t28 * t247 * t342;
    let t8279 = t8276 * t8278;
    let t8281 = t3509 * t370;
    (t8245, t8263, t8266, t8276, t8278, t8279, t8281)
}
