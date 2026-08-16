//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk808;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk809;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk810;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk811;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta174(t16: f64, t9688: f64, t2386: f64, t625: f64, t2385: f64, t686: f64, t781: f64, t685: f64, t120: f64, t118: f64, t123: f64, t116: f64, t2397: f64, t693: f64, t119: f64, t133: f64, t739: f64, t746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9689, t9691, t9692, t9694, t9695, t9697) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk808(t16, t9688, t2386, t625, t2385, t686, t781, t685, t120);
        let t9698 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk809(t118, t9697);
        let (t9701, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk810(t123, t116, t16, t2397, t9691, t693, t9694, t119, t133, t625);
        let t9711 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk811(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let t9713 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk812(t739, t746, t9711);
    (t9689, t9692, t9695, t9697, t9698, t9701, t9702, t9704, t9706, t9709, t9711, t9713)
}
