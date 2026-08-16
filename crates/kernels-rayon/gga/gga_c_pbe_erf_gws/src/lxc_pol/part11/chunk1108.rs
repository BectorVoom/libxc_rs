//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1108/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1108(t40768: f64, t40771: f64, t40773: f64, t40783: f64, t31443: f64, t3354: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47728 = 32.0_f64 / 15.0_f64 * t40768;
    let t47729 = 32.0_f64 / 45.0_f64 * t40771;
    let t47730 = 128.0_f64 / 45.0_f64 * t40773;
    let t47731 = 32.0_f64 / 15.0_f64 * t40783;
    let t47732 = 16.0_f64 / 45.0_f64 * t31443;
    let t47733 = t3354 * t3354;
    (t47728, t47729, t47730, t47731, t47732, t47733)
}
