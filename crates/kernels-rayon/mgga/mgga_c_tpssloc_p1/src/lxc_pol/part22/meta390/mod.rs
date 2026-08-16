//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1660;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1661;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1662;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1663;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1664;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1665;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1666;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta390(t18050: f64, t18168: f64, t1068: f64, t1070: f64, t17194: f64, t17197: f64, t17198: f64, t17202: f64, t17209: f64, t17301: f64, t17303: f64, t17306: f64, t17372: f64, t17374: f64, t17377: f64, t17379: f64, t17425: f64, t17427: f64, t17561: f64, t17563: f64, t17568: f64, t193: f64, t336: f64, t4696: f64, t4700: f64, t4701: f64, t17490: f64, t17504: f64, t17506: f64, t17509: f64, t17512: f64, t17515: f64, t17519: f64, t17523: f64, t17526: f64, t17530: f64, t17929: f64, t17932: f64, t17936: f64, t17940: f64, t17942: f64, t17944: f64, t17946: f64, t17950: f64, t17953: f64, t17957: f64, t25: f64, t265: f64, t394: f64, t17133: f64, t1074: f64, t1408: f64, t1409: f64, t1642: f64, t16557: f64, t16558: f64, t17141: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t5397: f64, t5398: f64, t5669: f64, t5955: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t5972: f64, t690: f64, t11147: f64, t5392: f64, t11145: f64, t123: f64, t11153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18169, t18173) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1660(t18050, t18168, t1068, t1070, t17194, t17197, t17198, t17202, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t193, t336, t4696, t4700, t4701);
        let t18174 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1661(t17490, t17504, t17506, t17509, t17512, t17515, t17519, t17523, t17526, t17530, t17929, t17932, t17936, t17940, t17942, t17944, t17946, t17950, t17953, t17957);
        let (t18176, t18188) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1662(t25, t265, t394, t17133, t18173, t18174, t1074, t1408, t1409, t1642, t16557, t16558, t17141, t396, t3966, t40, t4324, t4705, t5397, t5398, t5669, t5955, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t18196 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1663(t16557);
        let t18203 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1664(t5972, t690);
        let (t18205, t18206) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1665(t11147, t5392, t607);
        let (t18207, t18208) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1666(t11145, t18206, t123);
        let (t18210, t18211) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1667(t11153, t5392, t607);
    (t18169, t18176, t18188, t18196, t18203, t18205, t18206, t18207, t18208, t18210, t18211)
}
