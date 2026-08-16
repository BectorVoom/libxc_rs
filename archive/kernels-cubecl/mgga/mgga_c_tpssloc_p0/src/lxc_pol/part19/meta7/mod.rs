//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta7 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk54;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk55;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk56;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk57;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk58;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk59;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk60;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk61;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk62;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta7<F: Float>(t60: F, t118: F, t67: F, t117: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t119 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk54::<F>();
        let t120 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk55::<F>(t119);
        let t121 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk56::<F>(t60);
        let (t122, t123) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk57::<F>(t120, t121, t118);
        let t125 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk58::<F>(t123);
        let t126 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk59::<F>(t123);
        let (t129, t131) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk60::<F>(t123, t67);
        let (t132, t133) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk61::<F>(t117, t131);
        let t134 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk62::<F>(t61);
    (t119, t120, t121, t122, t123, t125, t126, t129, t131, t132, t133, t134)
}
