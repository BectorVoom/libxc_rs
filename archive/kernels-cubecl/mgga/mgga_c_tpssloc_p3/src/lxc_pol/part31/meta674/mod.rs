//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2036;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2037;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2038;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2039;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta674<F: Float>(t102386: F, t1266: F, t1393: F, t19461: F, t2040: F, t2075: F, t2314: F, t24432: F, t24995: F, t26161: F, t26558: F, t26872: F, t26878: F, t26880: F, t27171: F, t28030: F, t28943: F, t28951: F, t28952: F, t29219: F, t29241: F, t29380: F, t4028: F, t4034: F, t5457: F, t652: F, t672: F, t6876: F, t7050: F, t7156: F, t75210: F, t7685: F, t91655: F, t96709: F, t97902: F, t97933: F, t12725: F, t1774: F, t19451: F, t19456: F, t20100: F, t20136: F, t20143: F, t22574: F, t23938: F, t26977: F, t27147: F, t27150: F, t27163: F, t27170: F, t27226: F, t28002: F, t28821: F, t28830: F, t29247: F, t32193: F, t5494: F, t6287: F, t7042: F, t7056: F, t7057: F, t7061: F, t7220: F, t7458: F, t7796: F, t7802: F, t83886: F, t101091: F, t101134: F, t102105: F, t102320: F, t102366: F, t102988: F, t100930: F, t1458: F, t16521: F, t16524: F, t19534: F, t20162: F, t20173: F, t20181: F, t2039: F, t24465: F, t27254: F, t27281: F, t28893: F, t29422: F, t29425: F, t3941: F, t4072: F, t5456: F, t5493: F, t55353: F, t577: F, t66958: F, t671: F, t7801: F, t7956: F, t84033: F, t29430: F, t576: F, t1858: F, t7945: F, t29395: F, t580: F, t2098: F, t6483: F, t101021: F, t1396: F, t1398: F, t1852: F, t27286: F, t3: F, t6471: F, t7240: F, t94113: F, t94118: F, t94120: F, t94122: F, t100976: F) -> F {
        let t103029 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2036::<F>(t102386, t1266, t1393, t19461, t2040, t2075, t2314, t24432, t24995, t26161, t26558, t26872, t26878, t26880, t27171, t28030, t28943, t28951, t28952, t29219, t29241, t29380, t4028, t4034, t5457, t652, t672, t6876, t7050, t7156, t75210, t7685, t91655, t96709, t97902, t97933);
        let t103070 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2037::<F>(t12725, t1774, t19451, t19456, t20100, t20136, t20143, t22574, t23938, t26977, t27147, t27150, t27163, t27170, t27226, t28002, t28821, t28830, t29247, t32193, t4028, t5494, t6287, t652, t7042, t7056, t7057, t7061, t7220, t7458, t7796, t7802, t83886);
        let (t103073, t103088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2038::<F>(t101091, t101134, t102105, t102320, t102366, t102988, t103029, t103070, t100930, t1458, t16521, t16524, t19534, t20162, t20173, t20181, t2039, t24465, t27170, t27254, t27281, t28893, t28951, t29422, t29425, t3941, t4072, t5456, t5493, t55353, t577, t66958, t671, t7056, t7801, t7956, t84033);
        let t103102 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2039::<F>(t29430, t576, t1858, t7945, t29395, t580, t2098, t6483, t101021, t103073, t103088, t1396, t1398, t1852, t27286, t3, t6471, t7240, t94113, t94118, t94120, t94122);
        let tv4rho3sigma7 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2040::<F>(t100976, t103102);
    tv4rho3sigma7
}
