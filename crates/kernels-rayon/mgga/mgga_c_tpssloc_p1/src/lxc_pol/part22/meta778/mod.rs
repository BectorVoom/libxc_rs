//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta778 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2663;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2664;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2665;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2666;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2667;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta778(t1307: f64, t20563: f64, t12211: f64, t20516: f64, t20501: f64, t3726: f64, t54042: f64, t6390: f64, t20479: f64, t3866: f64, t16336: f64, t6427: f64, t1824: f64, t6414: f64, t119: f64, t1315: f64, t16101: f64, t16224: f64, t16305: f64, t16321: f64, t19994: f64, t20433: f64, t20570: f64, t210: f64, t221: f64, t3778: f64, t3783: f64, t3803: f64, t3807: f64, t40168: f64, t5301: f64, t5308: f64, t54614: f64, t6415: f64, t6420: f64, t74355: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t53783: f64, t53788: f64, t53797: f64, t73958: f64, t73959: f64, t73960: f64, t73961: f64, t73962: f64, t73968: f64, t73969: f64, t74013: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39324: f64, t39327: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t54315: f64, t54317: f64, t74017: f64, t74024: f64, t74026: f64, t74027: f64, t74028: f64, t39360: f64, t39364: f64, t39373: f64, t39384: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t74036: f64, t74040: f64, t74041: f64, t74042: f64, t74043: f64, t74044: f64, t74046: f64, t54412: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t39490: f64, t39496: f64, t54401: f64, t54403: f64, t54409: f64, t74056: f64, t74057: f64, t74073: f64, t74075: f64, t74078: f64, t74086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74389, t74393, t74395, t74401, t74403, t74405) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2663(t1307, t20563, t12211, t20516, t20501, t3726, t54042, t6390, t20479, t3866, t16336, t6427);
        let (t74415, t74428) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2664(t1824, t6414, t119, t1315, t16101, t16224, t16305, t16321, t19994, t20433, t20570, t210, t221, t3778, t3783, t3803, t3807, t40168, t5301, t5308, t54614, t6415, t6420, t6427, t74355, t74389, t74393, t74395, t74401, t74403, t74405);
        let t74466 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2665(t39249, t39256, t39261, t39266, t39304, t53783, t53788, t53797, t73958, t73959, t73960, t73961, t73962, t73968, t73969, t74013);
        let t74467 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2666(t39309, t39312, t39316, t39320, t39324, t39327, t39338, t39346, t39349, t39356, t54315, t54317, t74017, t74024, t74026, t74027, t74028);
        let t74469 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2667(t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t39411, t74036, t74040, t74041, t74042, t74043, t74044, t74046);
        let (t74470, t74471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2668(t54412, t39463, t39468, t39472, t39476, t39483, t39490, t39496, t54401, t54403, t54409, t74056, t74057, t74073, t74075, t74078, t74086);
    (t74389, t74415, t74428, t74466, t74467, t74469, t74470, t74471)
}
