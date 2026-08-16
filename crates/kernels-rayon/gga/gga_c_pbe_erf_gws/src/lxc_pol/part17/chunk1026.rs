//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1026/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1026(t3078: f64, t9246: f64, t3077: f64, t3103: f64, t840: f64, t3307: f64, t338: f64, t892: f64, t376: f64, t8574: f64, t353: f64, t9169: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9247 = t9246 * t3078;
    let t9249 = 7.0_f64 / 144.0_f64 * t3077 * t9247;
    let t9253 = 7.0_f64 / 144.0_f64 * t840 * t3103;
    let t9255 = t338 * t892 * t3307;
    let t9258 = t376 * t8574;
    let t9260 = t338 * t353 * t9258;
    let t9263 = param_a_c * t9169;
    (t9249, t9253, t9255, t9258, t9260, t9263)
}
