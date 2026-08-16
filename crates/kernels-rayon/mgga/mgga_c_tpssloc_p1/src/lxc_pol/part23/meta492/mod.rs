//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1509;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1510;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta492(t54460: f64, t54462: f64, t54467: f64, t57235: f64, t54477: f64, t39655: f64, t39658: f64, t39660: f64, t39844: f64, t39856: f64, t40224: f64, t40228: f64, t40230: f64, t1347: f64, t1819: f64, t1821: f64, t19708: f64, t19715: f64, t20416: f64, t20536: f64, t20544: f64, t20547: f64, t20550: f64, t225: f64, t3843: f64, t40253: f64, t5278: f64, t5279: f64, t546: f64, t548: f64, t6347: f64, t6404: f64, t6408: f64, t6411: f64, t79921: f64, t79984: f64, t80021: f64, t80101: f64, t80102: f64, t80104: f64, t80105: f64, t80108: f64, t80109: f64, t80111: f64, t550: f64, t1336: f64, t1380: f64, t19654: f64, t19739: f64, t19743: f64, t19810: f64, t19815: f64, t20473: f64, t20554: f64, t20568: f64, t20632: f64, t20638: f64, t20643: f64, t20645: f64, t3897: f64, t5234: f64, t5334: f64, t5344: f64, t5348: f64, t6415: f64, t6454: f64, t80085: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t80112, t80113, t80114, t80115, t80116, t80117) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1509(t54460, t54462, t54467, t57235, t54477, t39655, t39658, t39660, t39844, t39856, t40224, t40228, t40230);
        let t80150 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1510(t1347, t1819, t1821, t19708, t19715, t20416, t20536, t20544, t20547, t20550, t225, t3843, t40253, t5278, t5279, t546, t548, t6347, t6404, t6408, t6411, t79921, t79984, t80021, t80101, t80102, t80104, t80105, t80108, t80109, t80111, t80117);
        let (t80151, t80164) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1511(t550, t80150, t1336, t1380, t19654, t19739, t19743, t19810, t19815, t20473, t20554, t20568, t20632, t20638, t20643, t20645, t3897, t5234, t5334, t5344, t5348, t6415, t6454, t80085);
    (t80112, t80113, t80114, t80115, t80116, t80151, t80164)
}
