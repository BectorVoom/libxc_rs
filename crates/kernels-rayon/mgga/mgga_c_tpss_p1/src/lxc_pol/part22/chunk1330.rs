//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1330/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1330(t1333: f64, t61870: f64, t19590: f64, t61873: f64, t18394: f64, t3532: f64, t2074: f64, t61877: f64, t640: f64, t18397: f64, t2100: f64, t13215: f64, t5527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65440 = t61870 * t1333;
    let t65442 = t61873 * t19590;
    let t65444 = t18394 * t3532;
    let t65446 = t1333 * t2074;
    let t65447 = t61877 * t65446;
    let t65449 = t3532 * t640;
    let t65450 = t18397 * t65449;
    let t65452 = t1333 * t2100;
    let t65453 = t18397 * t65452;
    let t65455 = t5527 * t13215;
    (t65440, t65442, t65444, t65447, t65450, t65453, t65455)
}
