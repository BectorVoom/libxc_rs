//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta59 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk359;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk360;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk361;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk362;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk363;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta59(t1519: f64, t218: f64, t1510: f64, t860: f64, t235: f64, t1499: f64, t226: f64, t255: f64, t812: f64, t858: f64, t1493: f64, t259: f64, t855: f64, t1464: f64, t1473: f64, t1476: f64, t1484: f64, t193: f64, t202: f64, t680: f64, t705: f64, t752: f64, t760: f64, t765: f64, t766: f64, t870: f64, t1409: f64, t883: f64, t882: f64, t123: f64, t881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1520, t1523, t1525, t1527) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk359(t1519, t218, t1510, t860, t235, t1499, t226, t255, t812);
        let t1528 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk360(t1527, t858);
        let t1530 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk361(t1493, t1520, t1528, t259, t855);
        let t1534 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk362(t1464, t1473, t1476, t1484, t1530, t193, t202, t680, t705, t752, t760, t765, t766, t870);
        let t1539 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk363(t1409, t883);
        let (t1540, t1541, t1543) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk364(t1539, t882, t123, t881);
    (t1520, t1523, t1525, t1527, t1528, t1530, t1534, t1539, t1540, t1541, t1543)
}
