//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta24 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk181;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk182;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk183;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk184;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk185;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk186;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta24(t470: f64, t68: f64, t225: f64, t358: f64, t425: f64, t453: f64, t455: f64, sigma2: f64, t46: f64, t47: f64, rho1: f64, t415: f64, t374: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t471, t475) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk181(t470, t68, t225, t358, t425, t453, t455);
        let (t476, t477, t478) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk182(t475, sigma2);
        let (t479, t480, t483) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk183(t477, t478, t46, t47, rho1);
        let t484 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk184(t479, t483);
        let t485 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk185(t471, t484);
        let t486 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk186(t415);
        let t488 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk187(t374, t375, t486);
    (t471, t475, t476, t477, t478, t479, t480, t483, t484, t485, t486, t488)
}
