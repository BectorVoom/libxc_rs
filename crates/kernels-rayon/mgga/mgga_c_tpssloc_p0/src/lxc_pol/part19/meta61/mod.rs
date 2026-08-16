//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta61 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk388;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk389;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk390;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk391;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta61(t496: f64, t68: f64, t1011: f64, t1209: f64, t1206: f64, t1215: f64, t491: f64, t357: f64, t475: f64, t1235: f64, t493: f64, t1201: f64, t470: f64, t494: f64, t1191: f64, t1236: f64, t1238: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1239, t1241, t1243, t1244) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk388(t496, t68, t1011, t1209, t1206);
        let (t1245, t1246) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk389(t1215, t491, t357, t475);
        let (t1247, t1249, t1251) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk390(t1245, t1246, t1235, t493, t1201, t1244, t470, t494);
        let t1252 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk391(t1241, t1251);
        let t1254 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk392(t1191, t1236, t1238, t1252, t498);
    (t1239, t1241, t1243, t1244, t1245, t1246, t1247, t1249, t1251, t1252, t1254)
}
