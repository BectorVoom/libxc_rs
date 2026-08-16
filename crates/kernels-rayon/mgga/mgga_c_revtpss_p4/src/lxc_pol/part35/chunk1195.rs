//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1195/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1195(t108516: f64, t108524: f64, t108537: f64, t108539: f64, t108554: f64, t108559: f64, t108562: f64, t114521: f64, t114525: f64, t114527: f64, t98141: f64, t98148: f64, t98161: f64, t98165: f64) -> f64 {
    let t115027 = -0.96037800584476210818e-1_f64 * t108516 + 0.12196800674228478774e-2_f64 * t108524 + 0.10289764348336736873e-1_f64 * t114521 + 7.0_f64 / 24.0_f64 * t108537 - 7.0_f64 / 8.0_f64 * t108539 + 3.0_f64 / 8.0_f64 * t114525 + 0.10289764348336736873e-1_f64 * t114527 - 0.17149607247227894789e-2_f64 * t108554 - 0.91464571985215438874e-3_f64 * t98141 + 0.65049603595885220128e-2_f64 * t98148 + 0.30492001685571196935e-4_f64 * t98161 - 0.68598428988911579154e-3_f64 * t108559 + 0.30492001685571196935e-3_f64 * t108562 - 0.27210710165601593065e0_f64 * t98165;
    t115027
}
