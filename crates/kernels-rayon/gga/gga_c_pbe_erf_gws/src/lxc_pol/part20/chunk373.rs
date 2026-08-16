//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 373/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk373(t1162: f64, t353: f64, t338: f64, t1115: f64, t1120: f64, t1146: f64, t335: f64, t833: f64, t842: f64, t844: f64) -> (f64, f64, f64) {
    let t1163 = t353 * t1162;
    let t1164 = t338 * t1163;
    let t1167 = t1115 * t833 / 96.0_f64 - t842 - t844 * t1120 / 48.0_f64 + t335 * t1146 / 96.0_f64 - t335 * t1164 / 96.0_f64;
    (t1163, t1164, t1167)
}
