//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta774 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2647;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2648;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta774(t28: f64, t1081: f64, t5966: f64, t584: f64, t15952: f64, t15955: f64, t18196: f64, t19559: f64, t20385: f64, t20390: f64, t2219: f64, t3672: f64, t39436: f64, t5142: f64, t517: f64, t71090: f64, zeta_threshold: f64, t157: f64, t73989: f64, t182: f64, t20675: f64, t3701: f64, t39305: f64, t1388: f64, t20077: f64, t20681: f64, t3918: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t5160: f64, t5187: f64, t53783: f64, t53788: f64, t53797: f64, t55224: f64, t73958: f64, t73959: f64, t73960: f64, t73961: f64, t73962: f64, t73968: f64, t73969: f64, t54312: f64, t39328: f64, t39339: f64, t39341: f64, t6347: f64, t54325: f64, t20416: f64, t3919: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t39360: f64, t5161: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73995, t73998, t74009) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2647(t28, t1081, t5966, t584, t15952, t15955, t18196, t19559, t20385, t20390, t2219, t3672, t39436, t5142, t517, t71090, zeta_threshold);
        let (t74011, t74013, t74017, t74020) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2648(t157, t73989, t74009, t182, t20675, t3701, t39305, t1388, t20077, t20681, t3918, t39249, t39256, t39261, t39266, t39304, t5160, t5187, t53783, t53788, t53797, t55224, t73958, t73959, t73960, t73961, t73962, t73968, t73969);
        let (t74024, t74026, t74027, t74028, t74036, t74037) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2649(t54312, t39328, t39339, t39341, t1388, t6347, t54325, t20416, t3918, t3919, t39338, t39346, t39349, t39356, t39360, t5161);
    (t73995, t73998, t74011, t74013, t74017, t74020, t74024, t74026, t74027, t74028, t74036, t74037)
}
