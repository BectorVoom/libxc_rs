//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2254/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2254(t1256: f64, t30812: f64, t104988: f64, t104990: f64, t20298: f64, t20302: f64, t21008: f64, t21022: f64, t21121: f64, t21161: f64, t21219: f64, t21228: f64, t26867: f64, t29047: f64, t29054: f64, t6640: f64, t97149: f64, t97232: f64) -> f64 {
    let t112491 = t30812 * t1256;
    let t112515 = -0.30488190661738479624e-2_f64 * t112491 + 0.10162730220579493208e-2_f64 * t104988 - 0.17149607247227894789e-2_f64 * t97149 * t21121 + 0.47637797908966374413e-3_f64 * t26867 * t21008 - 0.57165357490759649296e-3_f64 * t26867 * t21161 - 0.57165357490759649296e-3_f64 * t97232 * t6640 - 0.57165357490759649296e-3_f64 * t26867 * t21228 - 0.57165357490759649296e-3_f64 * t26867 * t21022 - 0.28582678745379824648e-3_f64 * t26867 * t21219 + t29047 * t29054 * t20302 / 108.0_f64 + t29047 * t29054 * t20298 / 36.0_f64 + t104990 / 648.0_f64;
    t112515
}
