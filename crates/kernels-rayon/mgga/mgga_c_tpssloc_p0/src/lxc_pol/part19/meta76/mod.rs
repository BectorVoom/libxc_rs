//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta76 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk453;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk454;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk455;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk456;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta76(t123: f64, t126: f64, t131: f64, t119: f64, t132: f64, t63: f64, t204: f64, t686: f64, t685: f64, t120: f64, t118: f64, t693: f64, t133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2385, t2386, t2387, t2388, t2390, t2391, t2393) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk453(t123, t126, t131, t119, t132, t63, t204, t686, t685, t120);
        let t2394 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk454(t118, t2393);
        let (t2397, t2398, t2400, t2402) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk455(t123, t131, t2387, t2390, t693, t119, t63);
        let t2403 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk456(t133, t2402);
        let t2405 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk457(t2388, t2391, t2394, t2398, t2400, t2403);
    (t2385, t2386, t2388, t2391, t2393, t2394, t2397, t2398, t2400, t2402, t2403, t2405)
}
