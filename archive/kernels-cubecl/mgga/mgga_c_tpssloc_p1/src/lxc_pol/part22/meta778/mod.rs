//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta778 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2663;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2664;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2665;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2666;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2667;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta778<F: Float>(t1307: F, t20563: F, t12211: F, t20516: F, t20501: F, t3726: F, t54042: F, t6390: F, t20479: F, t3866: F, t16336: F, t6427: F, t1824: F, t6414: F, t119: F, t1315: F, t16101: F, t16224: F, t16305: F, t16321: F, t19994: F, t20433: F, t20570: F, t210: F, t221: F, t3778: F, t3783: F, t3803: F, t3807: F, t40168: F, t5301: F, t5308: F, t54614: F, t6415: F, t6420: F, t74355: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t53783: F, t53788: F, t53797: F, t73958: F, t73959: F, t73960: F, t73961: F, t73962: F, t73968: F, t73969: F, t74013: F, t39309: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t54315: F, t54317: F, t74017: F, t74024: F, t74026: F, t74027: F, t74028: F, t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t74036: F, t74040: F, t74041: F, t74042: F, t74043: F, t74044: F, t74046: F, t54412: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39490: F, t39496: F, t54401: F, t54403: F, t54409: F, t74056: F, t74057: F, t74073: F, t74075: F, t74078: F, t74086: F) -> (F, F, F, F, F, F, F, F) {
        let (t74389, t74393, t74395, t74401, t74403, t74405) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2663::<F>(t1307, t20563, t12211, t20516, t20501, t3726, t54042, t6390, t20479, t3866, t16336, t6427);
        let (t74415, t74428) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2664::<F>(t1824, t6414, t119, t1315, t16101, t16224, t16305, t16321, t19994, t20433, t20570, t210, t221, t3778, t3783, t3803, t3807, t40168, t5301, t5308, t54614, t6415, t6420, t6427, t74355, t74389, t74393, t74395, t74401, t74403, t74405);
        let t74466 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2665::<F>(t39249, t39256, t39261, t39266, t39304, t53783, t53788, t53797, t73958, t73959, t73960, t73961, t73962, t73968, t73969, t74013);
        let t74467 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2666::<F>(t39309, t39312, t39316, t39320, t39324, t39327, t39338, t39346, t39349, t39356, t54315, t54317, t74017, t74024, t74026, t74027, t74028);
        let t74469 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2667::<F>(t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t39411, t74036, t74040, t74041, t74042, t74043, t74044, t74046);
        let (t74470, t74471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2668::<F>(t54412, t39463, t39468, t39472, t39476, t39483, t39490, t39496, t54401, t54403, t54409, t74056, t74057, t74073, t74075, t74078, t74086);
    (t74389, t74415, t74428, t74466, t74467, t74469, t74470, t74471)
}
