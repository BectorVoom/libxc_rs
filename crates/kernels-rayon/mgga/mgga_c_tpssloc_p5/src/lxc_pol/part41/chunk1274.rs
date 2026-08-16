//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1274/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1274(t1774: f64, t8189: f64, t29895: f64, t8262: f64, t26129: f64, t8180: f64, t1453: f64, t662: f64, t8184: f64, t4067: f64, t626: f64, t8266: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30274 = t1774 * t8189;
    let t30279 = t29895 * t8262;
    let t30281 = t8180 * t26129;
    let t30284 = t1453 * t662;
    let t30285 = t8184 * t30284;
    let t30288 = t8180 * t4067;
    let t30291 = t626 * t8266;
    (t30274, t30279, t30281, t30284, t30285, t30288, t30291)
}
