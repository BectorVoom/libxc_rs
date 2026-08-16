//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk703;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk704;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk705;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk706;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta103(t2764: f64, t273: f64, t241: f64, t63: f64, t281: f64, t283: f64, t699: f64, t909: f64, t976: f64, t891: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2802, t2810, t2815) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk703(t2764, t273);
        let (t2820, t2822, t2823, t2824, t2826) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk704(t241, t63, t281, t283, t699, t909, t976);
        let t2840 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk705(t891);
        let t2841 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk706(t2840);
        let t2842 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk707(t275, t2841);
    (t2802, t2810, t2815, t2820, t2822, t2823, t2824, t2826, t2840, t2841, t2842)
}
