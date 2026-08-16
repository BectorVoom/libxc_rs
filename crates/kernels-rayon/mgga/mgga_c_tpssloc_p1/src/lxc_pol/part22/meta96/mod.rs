//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk657;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk658;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk659;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk660;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk661;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta96(t157: f64, t2516: f64, t153: f64, t193: f64, t201: f64, t868: f64, t870: f64, t2369: f64, t2509: f64, t2512: f64, t761: f64, t172: f64, t753: f64, t763: f64, t2504: f64, t739: f64, t746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2517 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk657(t157, t2516);
        let (t2518, t2522) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk658(t153, t2517, t193, t201);
        let t2523 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk659(t868, t870);
        let (t2527, t2528) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk660(t2369, t2509, t2512);
        let (t2530, t2531) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk661(t2528, t761, t172, t753);
        let (t2532, t2535) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk662(t2531, t763, t2504, t739, t746);
    (t2517, t2518, t2522, t2523, t2527, t2528, t2530, t2531, t2532, t2535)
}
