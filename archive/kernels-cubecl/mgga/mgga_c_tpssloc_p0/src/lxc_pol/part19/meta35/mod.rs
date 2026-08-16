//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta35 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk247;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk248;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk249;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk250;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk251;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk252;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk253;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta35<F: Float>(t119: F, t697: F, t133: F, t688: F, t690: F, t694: F, t141: F, t683: F, t31: F, t32: F, t152: F, t40: F, t52: F, t185: F, t607: F, t73: F, t76: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t698 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk247::<F>(t119, t697);
        let t699 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk248::<F>(t133, t698);
        let t701 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk249::<F>(t688, t690, t694, t699);
        let t702 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk250::<F>(t141);
        let t703 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk251::<F>(t701, t702);
        let t705 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk252::<F>(t683, t703);
        let (t706, t707) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk253::<F>(t31, t32, t152);
        let (t708, t710, t717) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk254::<F>(t40, t52, t185, t607, t707, t73, t76, zeta_threshold);
    (t698, t699, t701, t702, t703, t705, t706, t707, t708, t710, t717)
}
