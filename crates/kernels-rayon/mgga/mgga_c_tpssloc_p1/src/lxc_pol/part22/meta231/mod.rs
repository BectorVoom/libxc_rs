//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1296;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1297;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1298;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1299;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta231(t123: f64, t116: f64, t16: f64, t2397: f64, t9691: f64, t693: f64, t9694: f64, t119: f64, t133: f64, t625: f64, t9689: f64, t9692: f64, t9695: f64, t9698: f64, t739: f64, t746: f64, t761: f64, t177: f64, t2508: f64, t2512: f64, t9490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9701, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1296(t123, t116, t16, t2397, t9691, t693, t9694, t119, t133, t625);
        let t9711 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1297(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let t9713 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1298(t739, t746, t9711);
        let (t9715, t9720) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1299(t761, t9713, t177, t2508);
        let t9722 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1300(t2512, t9490, t9720);
    (t9701, t9702, t9704, t9706, t9709, t9711, t9713, t9715, t9720, t9722)
}
