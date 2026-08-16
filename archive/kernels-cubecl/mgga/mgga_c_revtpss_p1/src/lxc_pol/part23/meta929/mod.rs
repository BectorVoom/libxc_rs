//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta929 (260520-c91 hierarchical CSE).
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
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3033;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3034;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3035;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3036;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3037;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3038;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3039;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3040;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3041;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3042;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta929<F: Float>(t24042: F, t994: F, t1000: F, t1076: F, t1079: F, t1096: F, t16305: F, t16371: F, t1652: F, t1696: F, t19403: F, t19429: F, t20178: F, t20191: F, t23603: F, t23621: F, t24047: F, t3047: F, t3052: F, t3063: F, t42067: F, t4764: F, t4773: F, t4941: F, t4947: F, t5016: F, t55413: F, t6245: F, t6251: F, t6351: F, t64550: F, t64629: F, t64636: F, t64845: F, t68170: F, t80274: F, t80310: F, t80349: F, t80391: F, t80425: F, t80458: F, t80490: F, t80519: F, t80557: F, t80592: F, t80622: F, t80654: F, t80691: F, t80724: F, t80764: F, t80798: F, t23959: F, t378: F, t1097: F, t11201: F, t16284: F, t16333: F, t16600: F, t19351: F, t19415: F, t19421: F, t20152: F, t20172: F, t20215: F, t20219: F, t23617: F, t24177: F, t3058: F, t3269: F, t4747: F, t4752: F, t4935: F, t4940: F, t5015: F, t53174: F, t53281: F, t6244: F, t6258: F, t6393: F, t64555: F, t64817: F, t68138: F, t78740: F, t995: F, t996: F, t4746: F, t6343: F, t11187: F, t16312: F, t1651: F, t16597: F, t1680: F, t19341: F, t19342: F, t19381: F, t19400: F, t19428: F, t19856: F, t20151: F, t20204: F, t20211: F, t23583: F, t24178: F, t3264: F, t3268: F, t4758: F, t4778: F, t4946: F, t53167: F, t6259: F, t68072: F, t68144: F, t68188: F, t999: F, t79862: F, t1073: F, t11121: F, t1695: F, t19380: F, t19385: F, t19425: F, t20188: F, t20195: F, t23607: F, t24068: F, t386: F, t42060: F, t43637: F, t53015: F, t6350: F, t6392: F, t64687: F, t64711: F, t64764: F, t78554: F, t79388: F, t1647: F, t1678: F, t6235: F, t11224: F, t16302: F, t16313: F, t16603: F, t19414: F, t20175: F, t20194: F, t20214: F, t23598: F, t24044: F, t24048: F, t24061: F, t33754: F, t55464: F, t64737: F, t68022: F, t80028: F, t989: F, t342: F, t16374: F, t19396: F, t20171: F, t23599: F, t24031: F, t4772: F, t4932: F, t53108: F, t56087: F, t64547: F, t68185: F, t78826: F, t79084: F, t1100: F, t1102: F, t198: F, t336: F, t5023: F, t78094: F, t78096: F, t78099: F, t78154: F, t78478: F, t78686: F, t78690: F, t78694: F, t78696: F, t78698: F, t80166: F, t80211: F, t78192: F, t78195: F, t78201: F, t78203: F, t78206: F, t78246: F, t78248: F, t78251: F, t78254: F, t78303: F, t78305: F, t78307: F, t78309: F, t78311: F, t78313: F, t78315: F, t78319: F, t78322: F, t78325: F, t78328: F, t78332: F, t78335: F, t19137: F, t19153: F, t27717: F, t5019: F, t63907: F, t78339: F, t78342: F, t78703: F, t78706: F, t78709: F, t78712: F, t78715: F, t78717: F, t30: F, t265: F, t393: F, t77472: F, t78403: F, t78414: F, t78444: F, t78475: F, t1106: F, t1468: F, t1469: F, t1587: F, t1704: F, t18280: F, t18281: F, t18884: F, t20236: F, t22670: F, t22671: F, t23436: F, t24192: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t5824: F, t5825: F, t605: F, t606: F, t6405: F, t76396: F, t76397: F, t77481: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1733: F, t68947: F, t20629: F, t5105: F, t16835: F, t6471: F, t20448: F, t5063: F, t58466: F, t6474: F, t24262: F, t44101: F) -> (F, F, F, F, F, F, F, F) {
        let t80819 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3033::<F>(t24042, t994, t1000, t1076, t1079, t1096, t16305, t16371, t1652, t1696, t19403, t19429, t20178, t20191, t23603, t23621, t24047, t3047, t3052, t3063, t42067, t4764, t4773, t4941, t4947, t5016, t55413, t6245, t6251, t6351, t64550, t64629, t64636, t64845, t68170, t80274, t80310, t80349, t80391, t80425, t80458, t80490, t80519, t80557, t80592, t80622, t80654, t80691, t80724, t80764, t80798);
        let t80869 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3034::<F>(t23959, t378, t1076, t1079, t1096, t1097, t11201, t16284, t16333, t1652, t16600, t1696, t19351, t19415, t19421, t20152, t20172, t20215, t20219, t23617, t24177, t3047, t3058, t3063, t3269, t4747, t4752, t4935, t4940, t4947, t5015, t53174, t53281, t6244, t6245, t6258, t6393, t64555, t64817, t68138, t78740, t995, t996);
        let t80918 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3035::<F>(t4746, t6343, t1000, t1079, t11187, t16284, t16312, t1651, t1652, t16597, t1680, t1696, t19341, t19342, t19381, t19400, t19428, t19856, t20151, t20172, t20204, t20211, t23583, t24177, t24178, t3264, t3268, t4752, t4758, t4773, t4778, t4941, t4946, t53167, t6244, t6245, t6259, t68072, t68144, t68188, t995, t999);
        let t80967 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3036::<F>(t378, t79862, t1000, t1073, t1076, t1079, t11121, t1652, t1695, t19380, t19385, t19403, t19425, t20188, t20195, t20204, t23607, t23959, t24047, t24068, t3063, t3269, t386, t42060, t43637, t4747, t4752, t4758, t4935, t4941, t5015, t53015, t6350, t6392, t64687, t64711, t64764, t78554, t79388, t995, t996, t999);
        let t81015 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3037::<F>(t1647, t6343, t1678, t6235, t1079, t1096, t1097, t11224, t16302, t16305, t16312, t16313, t1652, t16600, t16603, t19400, t19414, t19428, t20152, t20175, t20194, t20214, t23598, t24044, t24048, t24061, t3264, t33754, t4758, t4935, t4947, t5016, t53174, t55464, t6245, t6251, t6259, t64737, t68022, t80028, t989, t995, t996);
        let t81068 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3038::<F>(t24042, t342, t1076, t1079, t1096, t1097, t11201, t16284, t16302, t16374, t16603, t1695, t1696, t19381, t19396, t19421, t19429, t20151, t20171, t20195, t20204, t20219, t23599, t24031, t3058, t3063, t3268, t3269, t4747, t4764, t4772, t4778, t4932, t4935, t4946, t53108, t56087, t6235, t6258, t6259, t6392, t64547, t68185, t78826, t79084, t995, t996);
        let t81075 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3039::<F>(t1100, t1102, t198, t336, t5023, t78094, t78096, t78099, t78154, t78478, t78686, t78690, t78694, t78696, t78698, t80166, t80211, t80819, t80869, t80918, t80967, t81015, t81068);
        let (t81076, t81078) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3040::<F>(t78192, t78195, t78201, t78203, t78206, t78246, t78248, t78251, t78254, t78303, t78305, t78307, t78309, t78311, t78313, t78315, t78319, t78322, t78325, t78328, t78332, t78335);
        let t81088 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3041::<F>(t19137, t19153, t27717, t5019, t5023, t63907, t78339, t78342, t78703, t78706, t78709, t78712, t78715, t78717);
        let t81110 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3042::<F>(t30, t265, t393, t77472, t78403, t78414, t78444, t78475, t81075, t81076, t81078, t81088, t1106, t1468, t1469, t1587, t1704, t18280, t18281, t18884, t20236, t22670, t22671, t23436, t24192, t395, t4186, t45, t4560, t5028, t5824, t5825, t605, t606, t6405, t76396, t76397, t77481, t895, dens_threshold, rho0, zeta_threshold);
        let (t81123, t81128, t81130, t81132, t81134, t81136, t81138) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3043::<F>(t76396, t1733, t68947, t20629, t5105, t16835, t6471, t20448, t5063, t58466, t6474, t24262, t44101);
    (t81110, t81123, t81128, t81130, t81132, t81134, t81136, t81138)
}
