//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 523/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk523(t2220: f64, t376: f64, t338: f64, t840: f64, t894: f64, t892: f64, t939: f64, t2074: f64, t353: f64, t941: f64, t845: f64, t2201: f64, t329: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2221 = t2220 * t376;
    let t2222 = t338 * t2221;
    let t2225 = t840 * t894;
    let t2227 = t892 * t939;
    let t2228 = t338 * t2227;
    let t2231 = t376 * t2074;
    let t2232 = t353 * t2231;
    let t2233 = t338 * t2232;
    let t2236 = t840 * t941;
    let t2238 = t892 * t845;
    let t2239 = t338 * t2238;
    let t2242 = t329 * t2201;
    (t2222, t2225, t2227, t2228, t2231, t2232, t2233, t2236, t2238, t2239, t2242)
}
