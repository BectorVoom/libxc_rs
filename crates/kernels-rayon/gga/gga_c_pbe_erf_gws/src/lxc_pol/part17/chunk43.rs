//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 43/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk43(t95: f64, t96: f64) -> (f64, f64, f64, f64) {
    let t97 = t96 * t95;
    let t98 = f64::ln(2.0_f64);
    let t99 = t98 - 1.0_f64;
    let t100 = 2.0_f64 * t99;
    let t101 = t97 * t100;
    (t97, t99, t100, t101)
}
