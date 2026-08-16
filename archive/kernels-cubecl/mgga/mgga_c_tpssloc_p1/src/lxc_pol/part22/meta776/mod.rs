//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2651;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2652;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2653;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta776<F: Float>(t1799: F, t5356: F, t20684: F, t40611: F, t1390: F, t20675: F, t20531: F, t588: F, t592: F, t172: F, t20396: F, t763: F, t54411: F, t120: F, t20553: F, t12283: F, t20454: F, t20489: F, t12429: F, t1352: F, t16394: F, t1825: F, t19815: F, t19871: F, t19882: F, t19956: F, t19972: F, t19986: F, t20442: F, t3803: F, t3805: F, t3807: F, t5245: F, t5248: F, t5252: F, t5287: F, t56817: F, t16398: F, t20475: F, t19731: F, t3792: F, t16242: F, t16401: F, t19631: F, t19958: F, t19989: F, t20460: F, t20463: F, t20465: F, t20470: F, t20473: F, t5187: F, t5246: F, t5249: F, t5250: F, t550: F, t6394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74060, t74064, t74068, t74073, t74075, t74077) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2651::<F>(t1799, t5356, t20684, t40611, t1390, t20675, t20531, t588, t592, t172, t20396, t763);
        let (t74078, t74086, t74090, t74110, t74120) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2652::<F>(t74077, t54411, t120, t20553, t12283, t20454, t20489);
        let t74133 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2653::<F>(t12429, t1352, t16394, t1825, t19815, t19871, t19882, t19956, t19972, t19986, t20442, t3803, t3805, t3807, t5245, t5248, t5252, t5287, t56817, t74090, t74110, t74120);
        let (t74174, t74181) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2654::<F>(t16398, t20475, t19731, t3792, t12429, t16242, t16394, t16401, t19631, t19871, t19956, t19958, t19989, t20460, t20463, t20465, t20470, t20473, t3803, t3805, t5187, t5246, t5248, t5249, t5250, t550, t56817, t6394, t74120);
    (t74060, t74064, t74068, t74073, t74075, t74078, t74086, t74090, t74120, t74133, t74174, t74181)
}
