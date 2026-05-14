//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 229/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk229<F: Float>(t723: F, t836: F, t568: F, t315: F, t317: F, t323: F, t327: F, t770: F, t775: F, t776: F, t780: F, t784: F, t790: F, t792: F, t797: F, t798: F, t802: F, t807: F, t810: F, t813: F, t815: F, t819: F, t825: F, t827: F, t833: F) -> (F, F, F) {
    let t837 = t836 * t723;
    let t838 = t568 * t837;
    let t841 = 0.35750489951850426669e0 * t770 * t317 - 0.35750489951850426669e0 * t775 * t776 + 0.35750489951850426669e0 * t780 * t317 + 0.23833659967900284446e0 * t315 * t784 - 0.39722766613167140743e-1 * t790 * t792 - 0.35750489951850426669e0 * t797 * t798 - 0.11502877786176224903e1 * t802 * t327 + 0.11502877786176224903e1 * t807 * t810 - 0.23005755572352449806e1 * t813 * t815 - 0.15337170381568299871e1 * t323 * t819 + 0.25561950635947166451e0 * t825 * t827 + 0.23005755572352449806e1 * t833 * t838;
    (t837, t838, t841)
}
