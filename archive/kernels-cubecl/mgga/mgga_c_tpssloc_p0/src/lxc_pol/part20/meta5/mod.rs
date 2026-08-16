//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta5 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk39;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk40;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk41;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk42;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk43;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk44;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk45;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta5<F: Float>(t66: F, t80: F, t5: F, t24: F, t36: F, rho0: F, tau0: F, t25: F, t48: F, rho1: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83, t84, t85) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk39::<F>(t66, t80);
        let t86 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk40::<F>(t85);
        let t88 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk41::<F>(t5, t24, t86);
        let t89 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk42::<F>(t88);
        let t92 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk43::<F>(t36, rho0, tau0);
        let (t93, t94, t95) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk44::<F>(t25);
        let (t96, t100) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk45::<F>(t93, t95, t48, rho1, tau1);
    (t83, t84, t85, t86, t88, t89, t92, t93, t94, t95, t96, t100)
}
