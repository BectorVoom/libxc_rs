//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta458 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1336;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1337;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1338;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1339;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1340;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1341;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta458(t17156: f64, t5398: f64, t123: f64, t2768: f64, t2770: f64, t75847: f64, t41741: f64, t47787: f64, t59657: f64, t68442: f64, t76574: f64, t76578: f64, t76583: f64, t76587: f64, t76591: f64, t20217: f64, t4337: f64, t10277: f64, t75836: f64, t882: f64, t5677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76593, t76595) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1336(t17156, t5398, t123, t2768);
        let (t76597, t76599) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1337(t2770, t75847, t123, t2768);
        let (t76602, t76608) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1338(t41741, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599, t20217, t4337);
        let t76610 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1339(t123, t2768, t76608);
        let (t76612, t76614) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1340(t10277, t75836, t123, t882);
        let (t76616, t76618) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1341(t5398, t5677, t123, t882);
    (t76593, t76595, t76597, t76599, t76602, t76608, t76610, t76612, t76614, t76616, t76618)
}
