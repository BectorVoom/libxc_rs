//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1237/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1237(t1313: f64, t619: f64, t77: f64, t3418: f64, t582: f64, t1317: f64, t615: f64, t3486: f64, t84: f64, t1290: f64, t1976: f64, t3426: f64, t578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19388 = t77 * t1313 * t619;
    let t19396 = t3418 * t582;
    let t19403 = t615 * t1317;
    let t19404 = t77 * t19403;
    let t19407 = t84 * t3486;
    let t19408 = t77 * t19407;
    let t19411 = t1976 * t1290;
    let t19414 = t578 * t3426;
    (t19388, t19396, t19403, t19404, t19407, t19408, t19411, t19414)
}
