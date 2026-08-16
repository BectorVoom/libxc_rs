//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2236/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2236(t104852: f64, t3767: f64, t3782: f64, t1224: f64, t139: f64, t29047: f64, t5052: f64, t3698: f64, t5047: f64, t16720: f64, t16725: f64, t17355: f64, t17420: f64, t17658: f64, t17669: f64, t17724: f64, t26867: f64, t26870: f64, t29054: f64, t29097: f64, t5407: f64, t97204: f64, t97232: f64) -> f64 {
    let t104853 = t3767 * t104852;
    let t104856 = t3782 * t104852;
    let t104863 = t29047 * t139 * t1224 * t5052 / 216.0_f64;
    let t104872 = t29047 * t139 * t3698 * t5047 / 324.0_f64;
    let t104876 = -0.85748036236139473944e-3_f64 * t26870 * t17724 - 0.57165357490759649296e-3_f64 * t97232 * t5407 - 0.57165357490759649296e-3_f64 * t26867 * t17669 - 0.11433071498151929859e-2_f64 * t104853 * t17658 + 0.57165357490759649296e-3_f64 * t104856 * t17355 + t97204 / 648.0_f64 - t104863 + 0.17149607247227894789e-2_f64 * t29097 * t17420 + t29047 * t29054 * t16720 / 36.0_f64 + t104872 + t29047 * t29054 * t16725 / 108.0_f64;
    t104876
}
