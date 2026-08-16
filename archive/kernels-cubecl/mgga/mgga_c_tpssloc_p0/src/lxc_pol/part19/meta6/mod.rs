//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta6 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk46;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk47;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk48;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk49;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk50;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk51;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk52;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk53;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta6<F: Float>(t28: F, t100: F, t92: F, t96: F, t64: F, t89: F, t25: F, dens_threshold: F, rho0: F, zeta_threshold: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t101, t102, t103) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk46::<F>(t28);
        let (t106, t107) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk47::<F>(t101, t103, t100, t92, t96);
        let (t111, t109) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk48::<F>(t107, t64);
        let t112 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk49::<F>(t111);
        let t113 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk50::<F>(t112, t89);
        let t116 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk51::<F>(t25, dens_threshold, rho0, zeta_threshold);
        let t117 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk52::<F>(t116);
        let t118 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk53::<F>(t117, t67);
    (t101, t102, t103, t106, t107, t111, t109, t112, t113, t116, t117, t118)
}
