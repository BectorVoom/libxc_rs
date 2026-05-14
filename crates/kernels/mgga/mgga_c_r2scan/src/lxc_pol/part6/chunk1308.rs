//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1308/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1308<F: Float>(t20082: F, t20092: F, t20097: F, t20100: F, t20104: F, t20107: F, t24704: F, t24707: F, t24711: F, t24712: F, t24716: F, t24718: F, t24725: F, t6152: F, t7979: F, t2667: F, t6508: F) -> (F, F) {
    let t24727 = 0.7801399566048841707e0 * t6152 * t7979 - 0.38415120233790484326e0 * t20082 - 0.17465477326173296717e-1 * t24704 - 0.40752780427737692339e0 * t24707 - t24711 + 0.98781737744032673979e-1 * t24712 - 0.16463622957338778996e-1 * t24716 - 0.174549769648958674e0 * t24718 + 0.26023093918533882312e-2 * t20092 + 0.9541801103462423514e-2 * t20097 - 0.86743646395112941038e-3 * t20100 - 0.19043987679069580389e-1 * t20104 + 0.22084125774650235183e1 * t20107 + 0.19634394786159580877e0 * t24725;
    let t24732 = t2667 * t6508;
    (t24727, t24732)
}
