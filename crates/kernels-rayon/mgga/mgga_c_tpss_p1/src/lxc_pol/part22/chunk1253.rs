//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1253/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1253(t19466: f64, t19479: f64, t19491: f64, t18438: f64, t18452: f64, t18466: f64, t18934: f64, t18943: f64, t19471: f64, t19473: f64, t19477: f64, t19481: f64, t19483: f64, t19485: f64, t19489: f64, t19493: f64, t19495: f64) -> f64 {
    let t20142 = 7.0_f64 / 72.0_f64 * t19466;
    let t20146 = 7.0_f64 / 1152.0_f64 * t19479;
    let t20151 = 7.0_f64 / 288.0_f64 * t19491;
    let t20154 = t18934 + t18438 + t20142 + t19471 / 8.0_f64 - t19473 / 24.0_f64 + t19477 / 384.0_f64 + t20146 + t19481 / 192.0_f64 - t19483 / 768.0_f64 - t19485 / 768.0_f64 + t18452 + t18943 + t18466 + t19489 / 192.0_f64 + t20151 + 5.0_f64 / 192.0_f64 * t19493 - t19495 / 192.0_f64;
    t20154
}
