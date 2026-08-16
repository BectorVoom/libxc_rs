//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3862/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3862(t13847: f64, t22016: f64, t48731: f64, t73731: f64, t13804: f64, t22046: f64, t46416: f64, t48514: f64, t48516: f64, t48518: f64, t48527: f64, t48529: f64, t48531: f64, t48536: f64, t48540: f64, t48544: f64, t5673: f64) -> f64 {
    let t74232 = t48731 * t13847 * t73731 * t22016;
    let t74234 = -0.28582678745379824648e-4_f64 * t48514 + 0.1219527626469539185e-2_f64 * t48516 + 0.7558530601555998074e-1_f64 * t48518 + 0.10164000561857065645e-2_f64 * t48527 + 0.1219527626469539185e-2_f64 * t48529 - 0.30488190661738479624e-3_f64 * t48531 - 0.11433071498151929859e-3_f64 * t48536 - 0.57165357490759649296e-4_f64 * t48540 + 0.28582678745379824648e-4_f64 * t48544 - 0.12862205435420921092e-2_f64 * t13804 * t5673 * t22046 * t46416 - 0.15246000842785598468e-3_f64 * t74232;
    t74234
}
