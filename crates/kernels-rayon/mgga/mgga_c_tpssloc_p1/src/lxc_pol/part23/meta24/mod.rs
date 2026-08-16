//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta24 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk182;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk183;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk184;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk185;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk186;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk187;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk188;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk189;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta24(t46: f64, t47: f64, rho1: f64, t479: f64, t471: f64, t415: f64, t374: f64, t375: f64, t456: f64, t467: f64, t466: f64, t477: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t480 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk182(t46);
        let (t481, t483) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk183(t480, t47, rho1);
        let t484 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk184(t479, t483);
        let t485 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk185(t471, t484);
        let t486 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk186(t415);
        let t488 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk187(t374, t375, t486);
        let t491 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk188(t456, t467, t485, t488);
        let (t492, t493) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk189(t466, t491, t477, t68);
        let t494 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk190(t491, t493);
    (t480, t481, t483, t484, t485, t486, t488, t491, t492, t493, t494)
}
