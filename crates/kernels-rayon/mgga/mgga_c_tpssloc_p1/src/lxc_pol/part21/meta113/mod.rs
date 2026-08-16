//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk781;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk782;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk783;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk784;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk785;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta113(t2787: f64, t914: f64, t287: f64, t891: f64, t275: f64, t912: f64, t913: f64, t273: f64, t276: f64, t896: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2789, t2790, t2791) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk781(t2787, t914, t287, t891);
        let t2792 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk782(t275, t2791);
        let t2793 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk783(t912);
        let (t2794, t2796, t2798) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk784(t2793, t913, t2792, t273, t276);
        let t2799 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk785(t896);
        let (t2800, t2802, t2807) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk786(t2798, t2799, t2764, t2766, t2773, t2778, t2782);
    (t2789, t2790, t2791, t2792, t2793, t2794, t2796, t2798, t2799, t2800, t2802, t2807)
}
