//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk659;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk660;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk661;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta93<F: Float>(t659: F, t2341: F, t2248: F, t95: F, t102: F, t662: F, t103: F, t100: F, t2336: F, t657: F, t660: F, t92: F, t96: F, t109: F, t656: F, t2327: F, t2328: F, t2333: F, t64: F, t510: F, t177: F, t738: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2342, t2343, t2346, t2349) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk659::<F>(t659, t2341, t2248, t95, t102);
        let (t2350, t2351, t2354, t2355, t2358) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk660::<F>(t662, t2349, t2248, t103, t100, t2336, t2343, t2346, t657, t660, t92, t96);
        let (t2359, t2363) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk661::<F>(t109, t2358, t656, t2327, t2328, t2333, t64);
        let (t2364, t2367, t2368) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk662::<F>(t2363, t510, t177, t738);
    (t2342, t2349, t2350, t2351, t2354, t2355, t2358, t2359, t2363, t2364, t2367, t2368)
}
