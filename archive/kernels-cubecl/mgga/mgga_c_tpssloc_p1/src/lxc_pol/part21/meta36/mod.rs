//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta36 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk263;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk264;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk265;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk266;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk267;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk268;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk269;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta36<F: Float>(t688: F, t690: F, t694: F, t699: F, t141: F, t683: F, t31: F, t32: F, t152: F, t185: F, t607: F, t40: F, t52: F, t73: F, t76: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
        let t701 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk263::<F>(t688, t690, t694, t699);
        let t702 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk264::<F>(t141);
        let t703 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk265::<F>(t701, t702);
        let t705 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk266::<F>(t683, t703);
        let t706 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk267::<F>(t31, t32);
        let t707 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk268::<F>(t152, t706);
        let t708 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk269::<F>(t185, t607);
        let (t710, t717) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk270::<F>(t40, t52, t707, t708, t607, t73, t76, zeta_threshold);
    (t701, t702, t703, t705, t706, t707, t708, t710, t717)
}
