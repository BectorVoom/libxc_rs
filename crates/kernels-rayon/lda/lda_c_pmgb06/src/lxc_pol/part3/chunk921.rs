//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 921/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk921(t10524: f64, t115: f64, t247: f64, t413: f64, t113: f64, t642: f64, t8131: f64, t8193: f64, t342: f64, t4044: f64, t6007: f64, t3650: f64, t4359: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10525 = t10524 / 2.0_f64;
    let t10528 = 0.007532237109403992_f64 * t413 * t247 * t115;
    let t10531 = 0.015064474218807983_f64 * t113 * t642 * t115;
    let t10532 = 96.0_f64 * t8131;
    let t10533 = 60.0_f64 * t8193;
    let t10541 = t6007 * t4044 * t342;
    let t10544 = t4359 * t3650;
    (t10525, t10528, t10531, t10532, t10533, t10541, t10544)
}
