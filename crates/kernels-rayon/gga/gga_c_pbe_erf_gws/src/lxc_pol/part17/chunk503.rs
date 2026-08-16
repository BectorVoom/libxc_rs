//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 503/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk503(t2142: f64, t854: f64, t346: f64, t824: f64, t822: f64) -> (f64, f64, f64, f64) {
    let t2143 = t854 * t2142;
    let t2144 = 7.0_f64 / 144.0_f64 * t2143;
    let t2145 = t824 * t346;
    let t2146 = t822 * t2145;
    (t2143, t2144, t2145, t2146)
}
