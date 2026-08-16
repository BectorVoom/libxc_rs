//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1304/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1304(t1453: f64, t662: f64, t8184: f64, t4067: f64, t8180: f64, t626: f64, t8266: f64, t104: f64, t50: f64, t666: f64, t103: f64, t29900: f64, t8269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30284 = t1453 * t662;
    let t30285 = t8184 * t30284;
    let t30288 = t8180 * t4067;
    let t30291 = t626 * t8266;
    let t30293 = t50 * t104;
    let t30294 = t30293 * t666;
    let t30297 = t50 * t103;
    let t30298 = t30297 * t662;
    let t30301 = t29900 * t8269;
    (t30285, t30288, t30291, t30293, t30294, t30297, t30298, t30301)
}
