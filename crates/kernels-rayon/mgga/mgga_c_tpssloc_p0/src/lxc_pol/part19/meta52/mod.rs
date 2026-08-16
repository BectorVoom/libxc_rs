//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk341;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk342;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk343;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk344;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk345;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk346;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta52(t270: f64, t283: f64, t61: f64, t248: f64, t884: f64, t1000: f64, t1005: f64, t1020: f64, t1025: f64, t1032: f64, t1038: f64, t1041: f64, t350: f64, t378: f64, t964: f64, t973: f64, t997: f64, t349: f64, t225: f64, t382: f64, t386: f64, t68: f64, t1011: f64, t1014: f64, t1010: f64, t1022: f64, t381: f64, t357: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1043 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk341(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk342(t1043, t61);
        let t1046 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk343(t1044, t248, t884);
        let t1049 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk344(t1000, t1005, t1020, t1025, t1032, t1038, t1041, t1046, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk345(t1049, t349, t225, t382);
        let (t1053, t1055, t1057, t1058) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk346(t386, t68, t1011, t1014, t1010);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk347(t1022, t381, t357, t360);
    (t1043, t1044, t1046, t1049, t1050, t1052, t1053, t1055, t1057, t1058, t1059, t1060)
}
