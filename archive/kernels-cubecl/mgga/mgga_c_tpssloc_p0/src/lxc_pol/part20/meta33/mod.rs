//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk243;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk244;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk245;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk246;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk247;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk248;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk249;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta33<F: Float>(t609: F, t629: F, t642: F, t66: F, t80: F, t5: F, t601: F, t605: F, t86: F, t112: F, t111: F, t89: F, t107: F, t626: F, t106: F, t38: F, tau0: F, t606: F, t95: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t645 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk243::<F>(t609, t629, t642, t66, t80);
        let t649 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk244::<F>(t5, t601, t605, t645, t86);
        let t650 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk245::<F>(t112, t649);
        let t652 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk246::<F>(t111, t89);
        let (t654, t655, t656) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk247::<F>(t107, t626, t106);
        let t657 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk248::<F>(t38, tau0);
        let t659 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk249::<F>(t606);
        let (t660, t662) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk250::<F>(t659, t95);
    (t645, t649, t650, t652, t654, t655, t656, t657, t659, t660, t662)
}
