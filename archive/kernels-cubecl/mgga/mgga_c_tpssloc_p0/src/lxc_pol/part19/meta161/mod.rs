//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk778;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk779;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta161<F: Float>(t9384: F, t9385: F, t2341: F, t659: F, t2248: F, t9256: F, t95: F, t101: F, t102: F, t2350: F, t662: F, t2349: F, t2354: F, t103: F, t100: F, t2336: F, t2343: F, t2346: F, t657: F, t660: F, t92: F, t9374: F, t96: F, t109: F, t656: F, t64: F, t9358: F, t9359: F, t9361: F, t9363: F, t9367: F, t9371: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9386, t9389, t9390, t9393, t9394, t9398, t9400, t9403) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk778::<F>(t9384, t9385, t2341, t659, t2248, t9256, t95, t101, t102, t2350, t662, t2349);
        let (t9407, t9411) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk779::<F>(t2354, t9403, t9393, t103, t100, t2336, t2343, t2346, t657, t660, t92, t9374, t9386, t9390, t9394, t9400, t96);
        let (t9412, t9416) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk780::<F>(t109, t656, t9411, t64, t9358, t9359, t9361, t9363, t9367, t9371);
    (t9386, t9389, t9390, t9393, t9394, t9398, t9403, t9407, t9411, t9412, t9416)
}
