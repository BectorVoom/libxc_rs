//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1033/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1033(t9392: f64, t9424: f64, t9427: f64, t9430: f64, t9434: f64, t9437: f64, t3610: f64, t3974: f64, t6752: f64, t4500: f64, t806: f64, t3482: f64, t4488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12100 = 8.0_f64 / 135.0_f64 * t9392;
    let t12101 = 16.0_f64 / 45.0_f64 * t9424;
    let t12102 = 8.0_f64 / 15.0_f64 * t9427;
    let t12103 = 8.0_f64 / 15.0_f64 * t9430;
    let t12104 = 16.0_f64 / 45.0_f64 * t9434;
    let t12105 = 32.0_f64 / 405.0_f64 * t9437;
    let t12108 = 8.0_f64 / 9.0_f64 * t3974 * t6752 * t3610;
    let t12109 = t4500 * t806;
    let t12112 = 4.0_f64 / 9.0_f64 * t4488 * t12109 * t3482;
    (t12100, t12101, t12102, t12103, t12104, t12105, t12108, t12109, t12112)
}
