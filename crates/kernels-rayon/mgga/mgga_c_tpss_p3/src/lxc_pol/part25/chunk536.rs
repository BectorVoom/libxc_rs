//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 536/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk536(t841: f64, t845: f64, t281: f64, t844: f64, t269: f64, t267: f64, t270: f64, t2453: f64, t235: f64, t68: f64, t275: f64, t277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2476 = t841 * t845;
    let t2479 = t844 * t281;
    let t2480 = 1.0_f64 / t2479;
    let t2481 = t269 * t2480;
    let t2487 = 1.0_f64 / t270 / t267;
    let t2491 = 4.0_f64 / 9.0_f64 * t2453;
    let t2499 = 0.39862222222222222223e0_f64 * t2453;
    let t2504 = 1.0_f64/f64::sqrt(t267);
    let t2509 = t68 * t235;
    let t2511 = t275 * t2509 * t277;
    (t2476, t2480, t2481, t2487, t2491, t2499, t2504, t2509, t2511)
}
