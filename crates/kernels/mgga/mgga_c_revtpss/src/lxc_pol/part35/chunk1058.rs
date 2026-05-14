//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1058/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1058<F: Float>(t108516: F, t108524: F, t108537: F, t108539: F, t108554: F, t108559: F, t108562: F, t114521: F, t114525: F, t114527: F, t98141: F, t98148: F, t98161: F, t98165: F, t108566: F, t108570: F, t108576: F, t114541: F, t114543: F, t114545: F, t114547: F, t114549: F, t114551: F, t114553: F, t96321: F, t96322: F, t98174: F, t98200: F) -> (F, F) {
    let t115027 = -0.96037800584476210818e-1 * t108516 + 0.12196800674228478774e-2 * t108524 + 0.10289764348336736873e-1 * t114521 + 7.0 / 24.0 * t108537 - 7.0 / 8.0 * t108539 + 3.0 / 8.0 * t114525 + 0.10289764348336736873e-1 * t114527 - 0.17149607247227894789e-2 * t108554 - 0.91464571985215438874e-3 * t98141 + 0.65049603595885220128e-2 * t98148 + 0.30492001685571196935e-4 * t98161 - 0.68598428988911579154e-3 * t108559 + 0.30492001685571196935e-3 * t108562 - 0.27210710165601593065e0 * t98165;
    let t115040 = 0.16262400898971305032e-2 * t98174 - 0.15246000842785598468e-3 * t108566 - 0.15246000842785598468e-3 * t108570 + 0.12004725073059526352e-1 * t108576 - t114541 / 24.0 - t114543 / 2.0 - t96321 - 0.25724410870841842183e-2 * t114545 - 0.51448821741683684367e-1 * t114547 + 0.10289764348336736873e-1 * t114549 - 0.20579528696673473747e-1 * t114551 + 0.51448821741683684367e-2 * t114553 + 0.12196800674228478774e-3 * t98200 + t96322;
    (t115027, t115040)
}
