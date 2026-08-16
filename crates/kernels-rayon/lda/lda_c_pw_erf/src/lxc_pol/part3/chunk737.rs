//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 737/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk737(t203: f64, t4701: f64, t184: f64, t221: f64, t4039: f64, t493: f64, t3881: f64, t3885: f64, t3901: f64, t462: f64, t2849: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4702 = t203 * t4701;
    let t4703 = t4702 * t184;
    let t4705 = 2.0_f64 / 15.0_f64 * t4703 * t221;
    let t4707 = 4.0_f64 / 15.0_f64 * t493 * t4039;
    let t4708 = 8.0_f64 / 135.0_f64 * t3881;
    let t4709 = 8.0_f64 / 81.0_f64 * t3885;
    let t4710 = 16.0_f64 / 45.0_f64 * t3901;
    let t4711 = 4.0_f64 * t462;
    let t4712 = 12.0_f64 * t2849;
    let t4713 = -t4711 - t4712 + t2852;
    (t4702, t4703, t4705, t4707, t4708, t4709, t4710, t4713)
}
