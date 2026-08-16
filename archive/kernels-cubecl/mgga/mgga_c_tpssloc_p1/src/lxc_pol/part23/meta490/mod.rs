//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1498;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1499;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1500;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta490<F: Float>(t25: F, t17: F, t184: F, t79888: F, t57208: F, t6463: F, t57211: F, t54451: F, t74496: F, t1298: F, t19606: F, t20216: F, t3704: F, t39861: F, t5170: F, t5397: F, t75911: F, t79859: F, t79864: F, zeta_threshold: F, t28: F, t1302: F, t19618: F, t20390: F, t3711: F, t39877: F, t5178: F, t5966: F, t77953: F, t79873: F, t79878: F, t1297: F, t1390: F, t1845: F, t193: F, t20077: F, t20356: F, t3701: F, t3918: F, t39604: F, t39606: F, t39608: F, t39615: F, t39635: F, t39655: F, t533: F, t6347: F, t6460: F, t40343: F, t40347: F, t40350: F, t54633: F, t54639: F, t56465: F, t56469: F, t56484: F, t56491: F, t74702: F, t74724: F, t74741: F, t74745: F, t6330: F, t1315: F, t16101: F, t1799: F, t19781: F, t210: F, t214: F, t221: F, t3733: F, t40025: F, t40401: F, t40422: F, t5195: F, t54663: F, t54725: F, t56535: F, t56539: F, t74726: F, t74747: F, t74756: F, t79921: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t79942, t79946, t79947, t79952, t79953, t79954, t79970) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1498::<F>(t25, t17, t184, t79888, t57208, t6463, t57211, t54451, t74496, t1298, t19606, t20216, t3704, t39861, t5170, t5397, t75911, t79859, t79864, zeta_threshold);
        let (t79984, t79988) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1499::<F>(t28, t1302, t19618, t20390, t3711, t39877, t5178, t5966, t77953, t79873, t79878, t79970, t1297, t1390, t1845, t193, t20077, t20356, t3701, t3918, t39604, t39606, t39608, t39615, t39635, t39655, t533, t6347, t79942, t79946, t79947, t79952, t79953, t79954, zeta_threshold);
        let (t79993, t80019) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1500::<F>(t6460, t40343, t40347, t40350, t54633, t54639, t56465, t56469, t56484, t56491, t74702, t74724, t74741, t74745);
        let (t80021, t80047) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1501::<F>(t6330, t1315, t16101, t1799, t19781, t210, t214, t221, t3733, t40025, t40401, t40422, t5195, t54663, t54725, t56535, t56539, t6347, t74726, t74747, t74756, t79921, t79984);
    (t79942, t79946, t79952, t79953, t79954, t79984, t79988, t79993, t80019, t80021, t80047)
}
