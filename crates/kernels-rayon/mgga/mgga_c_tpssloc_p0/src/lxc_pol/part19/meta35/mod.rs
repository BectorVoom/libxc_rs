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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk247;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk248;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk249;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk250;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk251;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk252;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk253;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk254;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta35(t119: f64, t697: f64, t133: f64, t688: f64, t690: f64, t694: f64, t141: f64, t683: f64, t31: f64, t32: f64, t152: f64, t40: f64, t52: f64, t185: f64, t607: f64, t73: f64, t76: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t698 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk247(t119, t697);
        let t699 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk248(t133, t698);
        let t701 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk249(t688, t690, t694, t699);
        let t702 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk250(t141);
        let t703 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk251(t701, t702);
        let t705 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk252(t683, t703);
        let (t706, t707) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk253(t31, t32, t152);
        let (t708, t710, t717) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk254(t40, t52, t185, t607, t707, t73, t76, zeta_threshold);
    (t698, t699, t701, t702, t703, t705, t706, t707, t708, t710, t717)
}
