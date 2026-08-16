//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta94 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk663;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk664;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk665;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk666;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk667;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk668;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta94(t745: f64, t2368: f64, t746: f64, t761: f64, t118: f64, t187: f64, t677: f64, t763: f64, t200: f64, t262: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2369 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk663(t745);
        let t2371 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk664(t2368, t2369, t746);
        let t2373 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk665(t2371, t761);
        let t2374 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk666(t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk667(t677, t763);
        let t2377 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk668(t2374, t2375);
        let (t2378, t2379) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk669(t200, t262, t776);
    (t2369, t2371, t2373, t2374, t2375, t2377, t2378, t2379)
}
