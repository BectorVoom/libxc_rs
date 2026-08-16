//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk986;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk987;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk988;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta247<F: Float>(t3395: F, t3403: F, t1155: F, t1138: F, t3351: F, t1136: F, t3359: F, t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F, t423: F, t11177: F, t11365: F, t11366: F, t11400: F, t11405: F, t11409: F, t11410: F, t11415: F, t11420: F, t11421: F, t11426: F, t11429: F, t11430: F, t1148: F, t3327: F, t3332: F, t3352: F, t3357: F, t3360: F, t3376: F, t3401: F, t436: F, t11364: F, t300: F, t11128: F, t11133: F, t11179: F, t11182: F, t11184: F, t11187: F, t11194: F, t11272: F, t11280: F, t11288: F, t11290: F, t11296: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11433, t11434, t11437, t11441, t11455) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk986::<F>(t3395, t3403, t1155, t1138, t3351, t1136, t3359, t11135, t11137, t11139, t11141, t11143, t11150, t11156, t11161, t11165, t11170, t11174);
        let (t11470, t11472) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk987::<F>(t11135, t11137, t11139, t11141, t11143, t11150, t11156, t11161, t11165, t11170, t11174, t423);
        let t11473 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk988::<F>(t11177, t11365, t11366, t1138, t11400, t11405, t11409, t11410, t11415, t11420, t11421, t11426, t11429, t11430, t11434, t11437, t11441, t11455, t11472, t1148, t3327, t3332, t3352, t3357, t3360, t3376, t3401, t436);
        let (t11475, t11476) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk989::<F>(t11364, t11473, t300, t11128, t11133, t11179, t11182, t11184, t11187, t11194, t11272, t11280, t11288, t11290, t11296);
    (t11433, t11434, t11437, t11441, t11455, t11470, t11472, t11475, t11476)
}
