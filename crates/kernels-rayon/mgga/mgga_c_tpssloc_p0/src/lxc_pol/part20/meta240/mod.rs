//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1343;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1344;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1345;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta240(t123: f64, t116: f64, t16: f64, t2397: f64, t9691: f64, t693: f64, t9694: f64, t119: f64, t133: f64, t625: f64, t9689: f64, t9692: f64, t9695: f64, t9698: f64, t739: f64, t746: f64, t761: f64, t172: f64, t2448: f64, t763: f64, t177: f64, t2508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9701, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1343(t123, t116, t16, t2397, t9691, t693, t9694, t119, t133, t625);
        let t9711 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1344(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let t9713 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1345(t739, t746, t9711);
        let (t9715, t9716, t9717, t9718, t9720) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1346(t761, t9713, t172, t2448, t763, t177, t2508);
    (t9701, t9702, t9704, t9706, t9709, t9711, t9713, t9715, t9716, t9717, t9718, t9720)
}
