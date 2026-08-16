//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta32 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk233;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk234;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk235;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk236;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk237;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk238;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk239;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta32<F: Float>(t59: F, t625: F, t39: F, t44: F, t51: F, t615: F, t618: F, t621: F, t33: F, t40: F, t73: F, t52: F, t76: F, t607: F, t72: F, t609: F, t66: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t626 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk233::<F>(t59, t625);
        let (t627, t628) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk234::<F>(t626, t39, t44, t51, t615, t618, t621);
        let (t629, t632) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk235::<F>(t33, t628, t40);
        let t634 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk236::<F>(t632, t73);
        let t636 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk237::<F>(t52);
        let t638 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk238::<F>(t636, t76);
        let (t641, t642) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk239::<F>(t607, t634, t638, t72);
        let t645 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk240::<F>(t609, t629, t642, t66, t80);
    (t626, t627, t628, t629, t632, t634, t636, t638, t641, t642, t645)
}
