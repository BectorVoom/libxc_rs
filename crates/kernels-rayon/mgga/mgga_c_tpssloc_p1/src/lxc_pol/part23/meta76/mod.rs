//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta76 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk446;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk447;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk448;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk449;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk450;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk451;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta76(t2390: f64, t685: f64, t120: f64, t204: f64, t118: f64, t123: f64, t131: f64, t2387: f64, t693: f64, t119: f64, t63: f64, t133: f64, t2388: f64, t702: f64, t683: f64, t681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2391, t2393) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk446(t2390, t685, t120, t204);
        let t2394 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk447(t118, t2393);
        let (t2397, t2398, t2400, t2402) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk448(t123, t131, t2387, t2390, t693, t119, t63);
        let t2403 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk449(t133, t2402);
        let t2405 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk450(t2388, t2391, t2394, t2398, t2400, t2403);
        let (t2406, t2408) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk451(t2405, t702, t683);
        let t2409 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk452(t681);
    (t2391, t2393, t2394, t2397, t2398, t2400, t2402, t2403, t2405, t2406, t2408, t2409)
}
