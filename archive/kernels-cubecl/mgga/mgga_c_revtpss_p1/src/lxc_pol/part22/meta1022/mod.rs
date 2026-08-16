//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1022 (260520-c91 hierarchical CSE).
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
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3561;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3562;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3563;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3564;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3565;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3566;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3567;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3568;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3569;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3570;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3571;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1022<F: Float>(t1000: F, t1076: F, t1079: F, t1096: F, t11210: F, t16312: F, t16313: F, t16343: F, t1696: F, t19380: F, t19381: F, t19428: F, t20178: F, t225: F, t3047: F, t3058: F, t3271: F, t342: F, t385: F, t53119: F, t53174: F, t6351: F, t64831: F, t64835: F, t64841: F, t64845: F, t64896: F, t64997: F, t65102: F, t65150: F, t65196: F, t65239: F, t65279: F, t67584: F, t67633: F, t67684: F, t67723: F, t67768: F, t67813: F, t67859: F, t67905: F, t67946: F, t67989: F, t995: F, t996: F, t1078: F, t6258: F, t3057: F, t6343: F, t16284: F, t16292: F, t16295: F, t16302: F, t16327: F, t16352: F, t16371: F, t20188: F, t3060: F, t3066: F, t3269: F, t3325: F, t42052: F, t4764: F, t4773: F, t4778: F, t4941: F, t5016: F, t6392: F, t16239: F, t16287: F, t16362: F, t1647: F, t1652: F, t19342: F, t19351: F, t20152: F, t3052: F, t3063: F, t3270: F, t3326: F, t42067: F, t42107: F, t43637: F, t43642: F, t4747: F, t4947: F, t53058: F, t53157: F, t55421: F, t6245: F, t6350: F, t3046: F, t1073: F, t16243: F, t16333: F, t16340: F, t16344: F, t16600: F, t19856: F, t20151: F, t20204: F, t3067: F, t386: F, t4758: F, t53223: F, t55464: F, t65057: F, t5015: F, t11120: F, t11214: F, t16249: F, t16603: F, t20191: F, t3076: F, t3261: F, t43656: F, t52994: F, t53281: F, t55461: F, t55475: F, t6235: F, t6251: F, t64912: F, t64989: F, t20112: F, t15669: F, t1678: F, t1097: F, t11121: F, t16152: F, t16255: F, t16275: F, t16597: F, t19429: F, t20219: F, t33754: F, t4752: F, t53015: F, t53034: F, t53180: F, t6244: F, t1679: F, t994: F, t1071: F, t989: F, t11201: F, t11220: F, t16605: F, t19396: F, t3264: F, t3268: F, t4772: F, t4946: F, t65071: F, t65122: F, t999: F, t20230: F, t3336: F, t1100: F, t1102: F, t198: F, t336: F, t5023: F, t64467: F, t64471: F, t64475: F, t64483: F, t64567: F, t64592: F, t64626: F, t64661: F, t64694: F, t64722: F, t64753: F, t64788: F, t64822: F, t65402: F, t65404: F, t65408: F, t65413: F, t65415: F, t65417: F, t65419: F, t65421: F, t30: F, t265: F, t393: F, t63193: F, t63587: F, t63629: F, t63671: F, t63899: F, t63938: F, t64513: F, t64532: F, t1106: F, t13312: F, t1468: F, t1469: F, t15083: F, t16618: F, t1704: F, t18280: F, t18281: F, t18884: F, t20236: F, t2257: F, t2258: F, t2838: F, t3340: F, t395: F, t4186: F, t45: F, t5028: F, t51835: F, t5824: F, t5825: F, t605: F, t606: F, t60754: F, t6084: F, t63202: F, t63204: F, t63206: F, t6405: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t20400: F, t3543: F, t1765: F, t57861: F, t16784: F, t5207: F, t12248: F, t3385: F, t6439: F, t3367: F, t60717: F, t1120: F, t128: F, t2435: F, t6430: F, t6422: F) -> (F, F, F, F, F, F, F, F, F) {
        let t68006 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3561::<F>(t1000, t1076, t1079, t1096, t11210, t16312, t16313, t16343, t1696, t19380, t19381, t19428, t20178, t225, t3047, t3058, t3271, t342, t385, t53119, t53174, t6351, t64831, t64835, t64841, t64845, t64896, t64997, t65102, t65150, t65196, t65239, t65279, t67584, t67633, t67684, t67723, t67768, t67813, t67859, t67905, t67946, t67989, t995, t996);
        let t68038 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3562::<F>(t1078, t6258, t3057, t6343, t1076, t16284, t16292, t16295, t16302, t16312, t16313, t16327, t16352, t16371, t20188, t3060, t3066, t3269, t3325, t42052, t4764, t4773, t4778, t4941, t5016, t6392);
        let t68067 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3563::<F>(t1076, t16239, t16287, t16362, t1647, t1652, t19342, t19351, t20152, t20188, t3052, t3063, t3270, t3326, t42067, t42107, t43637, t43642, t4747, t4947, t53058, t53157, t55421, t6245, t6350);
        let t68097 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3564::<F>(t3046, t6343, t1000, t1073, t1076, t1096, t16243, t16333, t16340, t16344, t16352, t1652, t16600, t19856, t20151, t20204, t3067, t3269, t386, t4747, t4758, t4947, t5016, t53223, t55464, t65057);
        let t68130 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3565::<F>(t5015, t1076, t11120, t11214, t16249, t1652, t16603, t1696, t20191, t3058, t3066, t3076, t3261, t3269, t43656, t4758, t4778, t52994, t53281, t55461, t55475, t6235, t6245, t6251, t6350, t64912, t64989, t995, t996);
        let t68163 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3566::<F>(t20112, t342, t15669, t1678, t1076, t1079, t1097, t11121, t16152, t16255, t16275, t16333, t16597, t1696, t19429, t20219, t3058, t3060, t3063, t3270, t3325, t33754, t4752, t4941, t4947, t53015, t53034, t53174, t53180, t6244, t6392);
        let t68199 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3567::<F>(t1679, t994, t1071, t6235, t6343, t989, t1079, t1097, t11201, t11220, t16243, t16603, t16605, t19396, t20151, t20152, t20178, t20219, t3047, t3058, t3264, t3268, t3326, t4772, t4778, t4946, t6351, t65071, t65122, t995, t996, t999);
        let t68211 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3568::<F>(t20230, t3336, t1100, t1102, t198, t336, t5023, t64467, t64471, t64475, t64483, t64567, t64592, t64626, t64661, t64694, t64722, t64753, t64788, t64822, t65402, t65404, t65408, t65413, t65415, t65417, t65419, t65421, t68006, t68038, t68067, t68097, t68130, t68163, t68199);
        let t68231 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3569::<F>(t30, t265, t393, t63193, t63587, t63629, t63671, t63899, t63938, t64513, t64532, t68211, t1106, t13312, t1468, t1469, t15083, t16618, t1704, t18280, t18281, t18884, t20236, t2257, t2258, t2838, t3340, t395, t4186, t45, t5028, t51835, t5824, t5825, t605, t606, t60754, t6084, t63202, t63204, t63206, t6405, t895, dens_threshold, rho0, zeta_threshold);
        let (t68243, t68245, t68247, t68250, t68251, t68253) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3570::<F>(t20400, t3543, t1765, t57861, t16784, t5207, t12248, t3385, t6439, t3367, t60717, t1120, t128);
        let t68255 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3571::<F>(t2435, t6430);
        let t68257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3572::<F>(t2435, t6422);
    (t68231, t68243, t68245, t68247, t68250, t68251, t68253, t68255, t68257)
}
