//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3721/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3721(t17237: f64, t17381: f64, t5381: f64, t57270: f64, t57273: f64, t57290: f64, t57292: f64, t57295: f64, t57297: f64, t57299: f64, t57314: f64, t57316: f64, t57318: f64, t57321: f64, t57382: f64) -> f64 {
    let t70565 = -2.0_f64 / 243.0_f64 * t57270 + t57273 / 324.0_f64 + 0.85748036236139473944e-3_f64 * t57382 * t17381 + t57290 / 162.0_f64 + t57292 / 81.0_f64 - t57295 / 432.0_f64 - 0.15244095330869239812e-2_f64 * t57297 + 0.28582678745379824648e-3_f64 * t57299 - 0.1270341277572436651e-2_f64 * t5381 * t17237 - 0.30488190661738479624e-2_f64 * t57314 + 0.57165357490759649296e-3_f64 * t57316 - 0.30488190661738479624e-2_f64 * t57318 + 0.19055119163586549765e-2_f64 * t57321;
    t70565
}
