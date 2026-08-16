//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 548/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk548(t2352: f64, t898: f64, t353: f64, t338: f64, t2118: f64, t825: f64, t822: f64, t814: f64, t830: f64, t831: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2353 = t898 * t2352;
    let t2354 = t353 * t2353;
    let t2355 = t338 * t2354;
    let t2358 = t2118 * t825;
    let t2359 = t822 * t2358;
    let t2361 = t830 * t831 * t814;
    let t2362 = t829 * t2361;
    (t2353, t2354, t2355, t2358, t2359, t2362)
}
