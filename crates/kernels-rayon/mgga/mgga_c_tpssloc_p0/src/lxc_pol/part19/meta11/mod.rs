//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk88;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk89;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk90;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk91;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk92;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk93;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk94;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk95;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta11(t204: f64, t59: f64, t201: f64, t154: f64, t117: f64, t131: f64, t136: f64, t119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t205 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk88(t204, t59);
        let (t206, t207) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk89(t201, t154);
        let (t209, t210) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk90(t117, t131);
        let t212 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk91(t136);
        let t213 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk92(t212);
        let t214 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk93(t119, t213);
        let t215 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk94(t210, t214);
        let t218 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk95(t205, t207, t215);
    (t205, t206, t207, t209, t210, t212, t213, t214, t215, t218)
}
