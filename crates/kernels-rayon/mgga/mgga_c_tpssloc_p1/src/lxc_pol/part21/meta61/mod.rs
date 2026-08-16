//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta61 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk437;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk438;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk439;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk440;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk441;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta61(t1196: f64, t607: f64, t974: f64, t1190: f64, t225: f64, t68: f64, t484: f64, t1009: f64, t466: f64, t1011: f64, t476: f64, t478: f64, t1017: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1197, t1198, t1201) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk437(t1196, t607, t974, t1190, t225);
        let t1202 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk438(t1201, t68);
        let (t1203, t1206, t1207, t1208, t1209) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk439(t1202, t484, t1009, t466, t1011, t476);
        let t1210 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk440(t1209, t478);
        let t1212 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk441(t1017, t483, t1210);
        let t1213 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk442(t1207, t1212);
    (t1197, t1198, t1201, t1202, t1203, t1206, t1207, t1208, t1209, t1210, t1212, t1213)
}
