//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk532;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk533;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk534;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta73(t40: f64, t52: f64, t145: f64, t1471: f64, t185: f64, t157: f64, t182: f64, t1409: f64, t767: f64, t771: f64, zeta_threshold: f64, t210: f64, t214: f64, t785: f64, t787: f64, t797: f64, t252: f64, t119: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1472, t1473, t1474, t1476, t1484) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk532(t40, t52, t145, t1471, t185, t157, t182, t1409, t767, t771, zeta_threshold);
        let (t1489, t1492) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk533(t1484, t210, t214, t785, t787, t797);
        let (t1493, t1495) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk534(t1492, t252, t119, t1484);
        let (t1496, t1499) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk535(t1495, t210, t1492, t225);
    (t1472, t1473, t1474, t1476, t1484, t1489, t1492, t1493, t1495, t1496, t1499)
}
