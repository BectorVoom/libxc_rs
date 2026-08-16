//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta73 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk528;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk529;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk530;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk531;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk532;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk533;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta73(t1484: f64, t210: f64, t214: f64, t785: f64, t787: f64, t797: f64, t252: f64, t119: f64, t225: f64, t237: f64, t1464: f64, t1473: f64, t1476: f64, t680: f64, t705: f64, t752: f64, t760: f64, t765: f64, t824: f64, t228: f64, t230: f64, t232: f64, t819: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1489, t1492) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk528(t1484, t210, t214, t785, t787, t797);
        let (t1493, t1495) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk529(t1492, t252, t119, t1484);
        let (t1496, t1499) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk530(t1495, t210, t1492, t225);
        let (t1500, t1504) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk531(t1499, t237, t1464, t1473, t1476, t225, t680, t705, t752, t760, t765);
        let (t1506, t1509) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk532(t1484, t824, t1504, t228, t230);
        let t1510 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk533(t1509, t232);
        let t1512 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk534(t1510, t819, t820);
    (t1489, t1492, t1493, t1495, t1496, t1499, t1500, t1504, t1506, t1509, t1510, t1512)
}
