//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta32 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk234;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk235;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk236;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk237;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta32(t52: f64, t76: f64, t607: f64, t634: f64, t72: f64, t609: f64, t629: f64, t66: f64, t80: f64, t5: f64, t601: f64, t605: f64, t86: f64, t112: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t636 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk234(t52);
        let t638 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk235(t636, t76);
        let (t641, t642) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk236(t607, t634, t638, t72);
        let t645 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk237(t609, t629, t642, t66, t80);
        let (t649, t650) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk238(t5, t601, t605, t645, t86, t112);
    (t636, t638, t641, t642, t645, t649, t650)
}
