//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2651;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2652;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2653;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta776(t1799: f64, t5356: f64, t20684: f64, t40611: f64, t1390: f64, t20675: f64, t20531: f64, t588: f64, t592: f64, t172: f64, t20396: f64, t763: f64, t54411: f64, t120: f64, t20553: f64, t12283: f64, t20454: f64, t20489: f64, t12429: f64, t1352: f64, t16394: f64, t1825: f64, t19815: f64, t19871: f64, t19882: f64, t19956: f64, t19972: f64, t19986: f64, t20442: f64, t3803: f64, t3805: f64, t3807: f64, t5245: f64, t5248: f64, t5252: f64, t5287: f64, t56817: f64, t16398: f64, t20475: f64, t19731: f64, t3792: f64, t16242: f64, t16401: f64, t19631: f64, t19958: f64, t19989: f64, t20460: f64, t20463: f64, t20465: f64, t20470: f64, t20473: f64, t5187: f64, t5246: f64, t5249: f64, t5250: f64, t550: f64, t6394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74060, t74064, t74068, t74073, t74075, t74077) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2651(t1799, t5356, t20684, t40611, t1390, t20675, t20531, t588, t592, t172, t20396, t763);
        let (t74078, t74086, t74090, t74110, t74120) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2652(t74077, t54411, t120, t20553, t12283, t20454, t20489);
        let t74133 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2653(t12429, t1352, t16394, t1825, t19815, t19871, t19882, t19956, t19972, t19986, t20442, t3803, t3805, t3807, t5245, t5248, t5252, t5287, t56817, t74090, t74110, t74120);
        let (t74174, t74181) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2654(t16398, t20475, t19731, t3792, t12429, t16242, t16394, t16401, t19631, t19871, t19956, t19958, t19989, t20460, t20463, t20465, t20470, t20473, t3803, t3805, t5187, t5246, t5248, t5249, t5250, t550, t56817, t6394, t74120);
    (t74060, t74064, t74068, t74073, t74075, t74078, t74086, t74090, t74120, t74133, t74174, t74181)
}
