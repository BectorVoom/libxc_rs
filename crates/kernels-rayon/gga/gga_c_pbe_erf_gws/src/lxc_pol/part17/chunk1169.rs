//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1169/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1169(t2169: f64, t2200: f64, t329: f64, t2271: f64, t4383: f64, t822: f64, t2409: f64) -> (f64, f64, f64, f64) {
    let t20091 = t329 * t2200 * t2169;
    let t20112 = t2271 * t4383;
    let t20113 = t822 * t20112;
    let t20154 = t2169 * t2409;
    (t20091, t20112, t20113, t20154)
}
