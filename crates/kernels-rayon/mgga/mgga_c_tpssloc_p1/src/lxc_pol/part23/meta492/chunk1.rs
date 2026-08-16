//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1510/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1510(t1347: f64, t1819: f64, t1821: f64, t19708: f64, t19715: f64, t20416: f64, t20536: f64, t20544: f64, t20547: f64, t20550: f64, t225: f64, t3843: f64, t40253: f64, t5278: f64, t5279: f64, t546: f64, t548: f64, t6347: f64, t6404: f64, t6408: f64, t6411: f64, t79921: f64, t79984: f64, t80021: f64, t80101: f64, t80102: f64, t80104: f64, t80105: f64, t80108: f64, t80109: f64, t80111: f64, t80117: f64) -> f64 {
    let t80150 = -(t80101 + t80102 + t80104 + t80105 + t80108 + t80109 + t80111 + t80117) * t225 * t548 + 12.0_f64 * t20536 * t1821 - 72.0_f64 * t6404 * t6408 + 18.0_f64 * t6404 * t6411 + 240.0_f64 * t1819 * t20544 - 144.0_f64 * t19708 * t20547 + 12.0_f64 * t1819 * t20550 - 360.0_f64 * t546 * t40253 * t80021 + 360.0_f64 * t5278 * t19715 * t6347 - 36.0_f64 * t546 * t3843 * t79921 - 48.0_f64 * t5278 * t5279 * t20416 + 3.0_f64 * t546 * t1347 * t79984;
    t80150
}
