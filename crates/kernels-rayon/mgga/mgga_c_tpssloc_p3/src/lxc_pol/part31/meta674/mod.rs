//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2036;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2037;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2038;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2039;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta674(t102386: f64, t1266: f64, t1393: f64, t19461: f64, t2040: f64, t2075: f64, t2314: f64, t24432: f64, t24995: f64, t26161: f64, t26558: f64, t26872: f64, t26878: f64, t26880: f64, t27171: f64, t28030: f64, t28943: f64, t28951: f64, t28952: f64, t29219: f64, t29241: f64, t29380: f64, t4028: f64, t4034: f64, t5457: f64, t652: f64, t672: f64, t6876: f64, t7050: f64, t7156: f64, t75210: f64, t7685: f64, t91655: f64, t96709: f64, t97902: f64, t97933: f64, t12725: f64, t1774: f64, t19451: f64, t19456: f64, t20100: f64, t20136: f64, t20143: f64, t22574: f64, t23938: f64, t26977: f64, t27147: f64, t27150: f64, t27163: f64, t27170: f64, t27226: f64, t28002: f64, t28821: f64, t28830: f64, t29247: f64, t32193: f64, t5494: f64, t6287: f64, t7042: f64, t7056: f64, t7057: f64, t7061: f64, t7220: f64, t7458: f64, t7796: f64, t7802: f64, t83886: f64, t101091: f64, t101134: f64, t102105: f64, t102320: f64, t102366: f64, t102988: f64, t100930: f64, t1458: f64, t16521: f64, t16524: f64, t19534: f64, t20162: f64, t20173: f64, t20181: f64, t2039: f64, t24465: f64, t27254: f64, t27281: f64, t28893: f64, t29422: f64, t29425: f64, t3941: f64, t4072: f64, t5456: f64, t5493: f64, t55353: f64, t577: f64, t66958: f64, t671: f64, t7801: f64, t7956: f64, t84033: f64, t29430: f64, t576: f64, t1858: f64, t7945: f64, t29395: f64, t580: f64, t2098: f64, t6483: f64, t101021: f64, t1396: f64, t1398: f64, t1852: f64, t27286: f64, t3: f64, t6471: f64, t7240: f64, t94113: f64, t94118: f64, t94120: f64, t94122: f64, t100976: f64) -> f64 {
        let t103029 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2036(t102386, t1266, t1393, t19461, t2040, t2075, t2314, t24432, t24995, t26161, t26558, t26872, t26878, t26880, t27171, t28030, t28943, t28951, t28952, t29219, t29241, t29380, t4028, t4034, t5457, t652, t672, t6876, t7050, t7156, t75210, t7685, t91655, t96709, t97902, t97933);
        let t103070 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2037(t12725, t1774, t19451, t19456, t20100, t20136, t20143, t22574, t23938, t26977, t27147, t27150, t27163, t27170, t27226, t28002, t28821, t28830, t29247, t32193, t4028, t5494, t6287, t652, t7042, t7056, t7057, t7061, t7220, t7458, t7796, t7802, t83886);
        let (t103073, t103088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2038(t101091, t101134, t102105, t102320, t102366, t102988, t103029, t103070, t100930, t1458, t16521, t16524, t19534, t20162, t20173, t20181, t2039, t24465, t27170, t27254, t27281, t28893, t28951, t29422, t29425, t3941, t4072, t5456, t5493, t55353, t577, t66958, t671, t7056, t7801, t7956, t84033);
        let t103102 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2039(t29430, t576, t1858, t7945, t29395, t580, t2098, t6483, t101021, t103073, t103088, t1396, t1398, t1852, t27286, t3, t6471, t7240, t94113, t94118, t94120, t94122);
        let tv4rho3sigma7 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2040(t100976, t103102);
    tv4rho3sigma7
}
