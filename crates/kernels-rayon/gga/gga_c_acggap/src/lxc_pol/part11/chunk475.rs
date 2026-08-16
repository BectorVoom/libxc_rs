//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 475/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk475(t2068: f64, t2268: f64, t515: f64, t570: f64, t495: f64, t579: f64, t336: f64, t2046: f64, t513: f64, t599: f64, t578: f64, t137: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2269 = t2068 * t2268;
    let t2271 = t570 * t515;
    let t2273 = t579 * t495;
    let t2274 = t336 * t2273;
    let t2275 = t2046 * t2274;
    let t2277 = t599 * t513;
    let t2278 = t336 * t2277;
    let t2279 = t578 * t2278;
    let t2282 = t336 * t535 * t137;
    (t2269, t2271, t2274, t2275, t2278, t2279, t2282)
}
