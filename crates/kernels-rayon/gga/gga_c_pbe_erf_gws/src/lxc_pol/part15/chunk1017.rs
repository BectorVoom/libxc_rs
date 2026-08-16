//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1017/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1017(t6617: f64, t2142: f64, t3113: f64, t6624: f64, t1136: f64, t6228: f64, t3028: f64, t817: f64, t1109: f64, t2106: f64, t1076: f64, t2108: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9140 = 35.0_f64 / 216.0_f64 * t6617;
    let t9142 = 7.0_f64 / 144.0_f64 * t3113 * t2142;
    let t9143 = 7.0_f64 / 288.0_f64 * t6624;
    let t9144 = t6228 * t1136;
    let t9145 = 35.0_f64 / 432.0_f64 * t9144;
    let t9147 = t3028 * t817;
    let t9150 = t1109 * t2106;
    let t9159 = t1076 * t2108;
    (t9140, t9142, t9143, t9145, t9147, t9150, t9159)
}
