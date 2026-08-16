//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 179/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk179(t723: f64, t836: f64, t568: f64, t315: f64, t317: f64, t323: f64, t327: f64, t770: f64, t775: f64, t776: f64, t780: f64, t784: f64, t790: f64, t792: f64, t797: f64, t798: f64, t802: f64, t807: f64, t810: f64, t813: f64, t815: f64, t819: f64, t825: f64, t827: f64, t833: f64) -> f64 {
    let t837 = t836 * t723;
    let t838 = t568 * t837;
    let t841 = 0.35750489951850426669e0_f64 * t770 * t317 - 0.35750489951850426669e0_f64 * t775 * t776 + 0.35750489951850426669e0_f64 * t780 * t317 + 0.23833659967900284446e0_f64 * t315 * t784 - 0.39722766613167140743e-1_f64 * t790 * t792 - 0.35750489951850426669e0_f64 * t797 * t798 - 0.11502877786176224903e1_f64 * t802 * t327 + 0.11502877786176224903e1_f64 * t807 * t810 - 0.23005755572352449806e1_f64 * t813 * t815 - 0.15337170381568299871e1_f64 * t323 * t819 + 0.25561950635947166451e0_f64 * t825 * t827 + 0.23005755572352449806e1_f64 * t833 * t838;
    t841
}
