//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1498;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1499;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1500;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta490(t25: f64, t17: f64, t184: f64, t79888: f64, t57208: f64, t6463: f64, t57211: f64, t54451: f64, t74496: f64, t1298: f64, t19606: f64, t20216: f64, t3704: f64, t39861: f64, t5170: f64, t5397: f64, t75911: f64, t79859: f64, t79864: f64, zeta_threshold: f64, t28: f64, t1302: f64, t19618: f64, t20390: f64, t3711: f64, t39877: f64, t5178: f64, t5966: f64, t77953: f64, t79873: f64, t79878: f64, t1297: f64, t1390: f64, t1845: f64, t193: f64, t20077: f64, t20356: f64, t3701: f64, t3918: f64, t39604: f64, t39606: f64, t39608: f64, t39615: f64, t39635: f64, t39655: f64, t533: f64, t6347: f64, t6460: f64, t40343: f64, t40347: f64, t40350: f64, t54633: f64, t54639: f64, t56465: f64, t56469: f64, t56484: f64, t56491: f64, t74702: f64, t74724: f64, t74741: f64, t74745: f64, t6330: f64, t1315: f64, t16101: f64, t1799: f64, t19781: f64, t210: f64, t214: f64, t221: f64, t3733: f64, t40025: f64, t40401: f64, t40422: f64, t5195: f64, t54663: f64, t54725: f64, t56535: f64, t56539: f64, t74726: f64, t74747: f64, t74756: f64, t79921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79942, t79946, t79947, t79952, t79953, t79954, t79970) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1498(t25, t17, t184, t79888, t57208, t6463, t57211, t54451, t74496, t1298, t19606, t20216, t3704, t39861, t5170, t5397, t75911, t79859, t79864, zeta_threshold);
        let (t79984, t79988) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1499(t28, t1302, t19618, t20390, t3711, t39877, t5178, t5966, t77953, t79873, t79878, t79970, t1297, t1390, t1845, t193, t20077, t20356, t3701, t3918, t39604, t39606, t39608, t39615, t39635, t39655, t533, t6347, t79942, t79946, t79947, t79952, t79953, t79954, zeta_threshold);
        let (t79993, t80019) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1500(t6460, t40343, t40347, t40350, t54633, t54639, t56465, t56469, t56484, t56491, t74702, t74724, t74741, t74745);
        let (t80021, t80047) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1501(t6330, t1315, t16101, t1799, t19781, t210, t214, t221, t3733, t40025, t40401, t40422, t5195, t54663, t54725, t56535, t56539, t6347, t74726, t74747, t74756, t79921, t79984);
    (t79942, t79946, t79952, t79953, t79954, t79984, t79988, t79993, t80019, t80021, t80047)
}
