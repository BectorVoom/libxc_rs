//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2210/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2210(t2327: f64, t8151: f64, t10301: f64, t29411: f64, t2247: f64, t29362: f64, t38: f64, t1923: f64, t25102: f64, t25110: f64, t25114: f64, t25117: f64, t25150: f64, t26782: f64, t26789: f64, t28089: f64, t29372: f64, t29375: f64, t29412: f64, t6954: f64, t6960: f64, t7575: f64, t7709: f64, t7719: f64, t8144: f64, t8147: f64) -> (f64, f64) {
    let t104163 = t8151 * t2327;
    let t104181 = t10301 * t29411;
    let t104185 = t2247 * t38 * t29362;
    let t104194 = -t25150 * t8147 / 6.0_f64 - t6954 * t29372 / 3.0_f64 - t6954 * t29375 / 3.0_f64 - t1923 * t26782 * t7719 / 6.0_f64 - t1923 * t7575 * t28089 / 3.0_f64 + t25117 * t8144 / 3.0_f64 + t7709 * t26789 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t104181 * t6960 + 5.0_f64 / 3.0_f64 * t104185 * t6960 + 5.0_f64 / 3.0_f64 * t29412 * t25110 + 5.0_f64 / 6.0_f64 * t29412 * t25114 + 2.0_f64 / 3.0_f64 * t25102 * t8144;
    (t104163, t104194)
}
