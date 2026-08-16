//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk626;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk627;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk628;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta90(t2371: f64, t761: f64, t118: f64, t187: f64, t677: f64, t763: f64, t200: f64, t262: f64, t123: f64, t126: f64, t131: f64, t119: f64, t132: f64, t63: f64, t204: f64, t686: f64, t685: f64, t120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2373, t2374) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk626(t2371, t761, t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk627(t677, t763);
        let (t2377, t2378, t2385, t2386, t2387, t2388, t2390) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk628(t2374, t2375, t200, t262, t123, t126, t131, t119, t132, t63, t204, t686);
        let (t2391, t2393) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk629(t2390, t685, t120, t204);
    (t2373, t2374, t2375, t2377, t2378, t2385, t2386, t2387, t2388, t2390, t2391, t2393)
}
