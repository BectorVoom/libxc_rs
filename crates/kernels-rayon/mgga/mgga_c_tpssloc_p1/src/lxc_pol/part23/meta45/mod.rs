//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk301;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk302;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk303;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk304;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta45(t386: f64, t68: f64, t1011: f64, t1014: f64, t1010: f64, t357: f64, t360: f64, t390: f64, t268: f64, t405: f64, t878: f64, t154: f64, t486: f64, t636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1053, t1055, t1057, t1058) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk301(t386, t68, t1011, t1014, t1010);
        let t1060 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk302(t357, t360);
        let (t1070, t1086) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk303(t390, t268, t405, t878);
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk304(t1086, t154, t486);
        let t1089 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk305(t636);
    (t1053, t1055, t1057, t1058, t1060, t1070, t1086, t1087, t1088, t1089)
}
