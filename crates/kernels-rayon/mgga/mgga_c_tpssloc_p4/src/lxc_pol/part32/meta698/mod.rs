//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2175;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2176;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2177;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2178;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2179;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2180;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2181;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2182;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2183;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta698(t26322: f64, t7708: f64, t91202: f64, t20004: f64, t26309: f64, t19945: f64, t19981: f64, t22833: f64, t19994: f64, t221: f64, t26284: f64, t19631: f64, t1998: f64, t236: f64, t6926: f64, t1361: f64, t22690: f64, t6330: f64, t80840: f64, t22792: f64, t6347: f64, t80900: f64, t80915: f64, t91387: f64, t93757: f64, t97394: f64, t97398: f64, t97400: f64, t97402: f64, t97404: f64, t97407: f64, t26318: f64, t91351: f64, t19844: f64, t6916: f64, t22804: f64, t28077: f64, t22779: f64, t28067: f64, t19924: f64, t26288: f64, t19919: f64, t91194: f64, t91198: f64, t20000: f64, t91361: f64, t28060: f64, t80940: f64, t80957: f64, t80971: f64, t91400: f64, t91403: f64, t91404: f64, t93760: f64, t97233: f64, t97268: f64, t97309: f64, t97349: f64, t97376: f64, t97392: f64, t19661: f64, t1992: f64, t22897: f64, t19736: f64, t22892: f64, t22893: f64, t28138: f64, t1336: f64, t1352: f64, t16060: f64, t19810: f64, t26404: f64, t26442: f64, t26456: f64, t26458: f64, t28152: f64, t3777: f64, t5234: f64, t5287: f64, t5344: f64, t544: f64, t553: f64, t7745: f64, t91065: f64, t91077: f64, t93795: f64, t93796: f64, t97172: f64, t97181: f64, t97189: f64, t97200: f64, t28116: f64, t81228: f64, t81326: f64, t6897: f64, t7700: f64, t90544: f64, t214: f64, t6434: f64, t1985: f64, t6907: f64, t22633: f64, t26215: f64, t90566: f64, t80722: f64, t80744: f64, t81264: f64, t90605: f64, t90609: f64, t90646: f64, t93438: f64, t93445: f64, t22635: f64, t26354: f64, t5353: f64, t26338: f64, t22751: f64, t28213: f64, t28210: f64, t28233: f64, t6883: f64, t1323: f64, t16439: f64, t19804: f64, t2006: f64, t22656: f64, t28107: f64, t28187: f64, t3882: f64, t568: f64, t6361: f64, t6461: f64, t6955: f64, t7750: f64, t81284: f64, t90702: f64, t90708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97410, t97412, t97414, t97416, t97419, t97423) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2175(t26322, t7708, t91202, t20004, t26309, t19945, t19981, t22833, t19994, t221, t26284, t19631, t1998, t236, t6926);
        let t97433 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2176(t1361, t22690, t6330, t80840, t22792, t6347, t80900, t80915, t91387, t93757, t97394, t97398, t97400, t97402, t97404, t97407, t97410, t97412, t97414, t97416, t97419, t97423);
        let (t97435, t97437, t97439, t97444, t97447) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2177(t26318, t7708, t91351, t19844, t6916, t22804, t28077, t22779, t28067, t1361, t19924, t26288);
        let (t97450, t97453, t97456, t97459, t97461, t97463) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2178(t1361, t19994, t26288, t19919, t221, t91194, t19924, t26284, t91198, t20000, t91361, t22779, t28060);
        let t97465 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2179(t80940, t80957, t80971, t91400, t91403, t91404, t93760, t97435, t97437, t97439, t97444, t97447, t97450, t97453, t97456, t97459, t97461, t97463);
        let (t97468, t97488, t97491) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2180(t97233, t97268, t97309, t97349, t97376, t97392, t97433, t97465, t19661, t1992, t22897, t19736);
        let t97496 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2181(t22892, t22893, t28138, t1336, t1352, t16060, t19810, t26404, t26442, t26456, t26458, t28152, t3777, t5234, t5287, t5344, t544, t553, t7745, t91065, t91077, t93795, t93796, t97172, t97181, t97189, t97200, t97468, t97488, t97491);
        let (t97503, t97509, t97511, t97513, t97516) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2182(t28116, t81228, t81326, t6897, t7700, t90544, t214, t6434, t1985, t6907, t22633, t26215, t90566);
        let (t97519, t97524) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2183(t80722, t80744, t81264, t90605, t90609, t90646, t93438, t93445, t97509, t97513, t97516, t1992, t22635, t26354, t5353);
        let (t97527, t97529, t97552) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2184(t22633, t26338, t90566, t22751, t28213, t28210, t28233, t6883, t1323, t16439, t19804, t2006, t22656, t28107, t28187, t3882, t568, t6361, t6461, t6955, t7750, t81284, t90702, t90708);
    (t97468, t97496, t97503, t97511, t97519, t97524, t97527, t97529, t97552)
}
