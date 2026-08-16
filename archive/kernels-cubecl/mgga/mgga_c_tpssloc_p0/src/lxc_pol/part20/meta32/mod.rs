//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta32 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk234;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk235;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk236;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk237;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk238;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk239;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk240;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk241;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta32<F: Float>(t43: F, t607: F, t55: F, t583: F, t61: F, t59: F, t39: F, t44: F, t51: F, t615: F, t33: F, t40: F, t73: F, t52: F, t76: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t618, t621, t625) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk234::<F>(t43, t607, t55, t583, t61);
        let t626 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk235::<F>(t59, t625);
        let (t627, t628) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk236::<F>(t626, t39, t44, t51, t615, t618, t621);
        let t629 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk237::<F>(t33, t628);
        let t632 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk238::<F>(t40);
        let t634 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk239::<F>(t632, t73);
        let t636 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk240::<F>(t52);
        let t638 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk241::<F>(t636, t76);
        let t642 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk242::<F>(t607, t634, t638, t72);
    (t618, t621, t625, t626, t627, t628, t629, t632, t634, t636, t638, t642)
}
