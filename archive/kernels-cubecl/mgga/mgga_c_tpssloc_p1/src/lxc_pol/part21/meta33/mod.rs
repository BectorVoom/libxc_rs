//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk240;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk241;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk242;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk243;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk244;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk245;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk246;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk247;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta33<F: Float>(t632: F, t73: F, t52: F, t76: F, t607: F, t72: F, t609: F, t629: F, t66: F, t80: F, t5: F, t601: F, t605: F, t86: F, t112: F, t111: F, t89: F, t107: F, t626: F, t106: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t634 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk240::<F>(t632, t73);
        let t636 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk241::<F>(t52);
        let t638 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk242::<F>(t636, t76);
        let (t641, t642) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk243::<F>(t607, t634, t638, t72);
        let t645 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk244::<F>(t609, t629, t642, t66, t80);
        let t649 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk245::<F>(t5, t601, t605, t645, t86);
        let t650 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk246::<F>(t112, t649);
        let t652 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk247::<F>(t111, t89);
        let (t654, t655, t656) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk248::<F>(t107, t626, t106);
    (t634, t636, t638, t641, t642, t645, t649, t650, t652, t654, t655, t656)
}
