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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1022(t1000: f64, t1076: f64, t1079: f64, t1096: f64, t11210: f64, t16312: f64, t16313: f64, t16343: f64, t1696: f64, t19380: f64, t19381: f64, t19428: f64, t20178: f64, t225: f64, t3047: f64, t3058: f64, t3271: f64, t342: f64, t385: f64, t53119: f64, t53174: f64, t6351: f64, t64831: f64, t64835: f64, t64841: f64, t64845: f64, t64896: f64, t64997: f64, t65102: f64, t65150: f64, t65196: f64, t65239: f64, t65279: f64, t67584: f64, t67633: f64, t67684: f64, t67723: f64, t67768: f64, t67813: f64, t67859: f64, t67905: f64, t67946: f64, t67989: f64, t995: f64, t996: f64, t1078: f64, t6258: f64, t3057: f64, t6343: f64, t16284: f64, t16292: f64, t16295: f64, t16302: f64, t16327: f64, t16352: f64, t16371: f64, t20188: f64, t3060: f64, t3066: f64, t3269: f64, t3325: f64, t42052: f64, t4764: f64, t4773: f64, t4778: f64, t4941: f64, t5016: f64, t6392: f64, t16239: f64, t16287: f64, t16362: f64, t1647: f64, t1652: f64, t19342: f64, t19351: f64, t20152: f64, t3052: f64, t3063: f64, t3270: f64, t3326: f64, t42067: f64, t42107: f64, t43637: f64, t43642: f64, t4747: f64, t4947: f64, t53058: f64, t53157: f64, t55421: f64, t6245: f64, t6350: f64, t3046: f64, t1073: f64, t16243: f64, t16333: f64, t16340: f64, t16344: f64, t16600: f64, t19856: f64, t20151: f64, t20204: f64, t3067: f64, t386: f64, t4758: f64, t53223: f64, t55464: f64, t65057: f64, t5015: f64, t11120: f64, t11214: f64, t16249: f64, t16603: f64, t20191: f64, t3076: f64, t3261: f64, t43656: f64, t52994: f64, t53281: f64, t55461: f64, t55475: f64, t6235: f64, t6251: f64, t64912: f64, t64989: f64, t20112: f64, t15669: f64, t1678: f64, t1097: f64, t11121: f64, t16152: f64, t16255: f64, t16275: f64, t16597: f64, t19429: f64, t20219: f64, t33754: f64, t4752: f64, t53015: f64, t53034: f64, t53180: f64, t6244: f64, t1679: f64, t994: f64, t1071: f64, t989: f64, t11201: f64, t11220: f64, t16605: f64, t19396: f64, t3264: f64, t3268: f64, t4772: f64, t4946: f64, t65071: f64, t65122: f64, t999: f64, t20230: f64, t3336: f64, t1100: f64, t1102: f64, t198: f64, t336: f64, t5023: f64, t64467: f64, t64471: f64, t64475: f64, t64483: f64, t64567: f64, t64592: f64, t64626: f64, t64661: f64, t64694: f64, t64722: f64, t64753: f64, t64788: f64, t64822: f64, t65402: f64, t65404: f64, t65408: f64, t65413: f64, t65415: f64, t65417: f64, t65419: f64, t65421: f64, t30: f64, t265: f64, t393: f64, t63193: f64, t63587: f64, t63629: f64, t63671: f64, t63899: f64, t63938: f64, t64513: f64, t64532: f64, t1106: f64, t13312: f64, t1468: f64, t1469: f64, t15083: f64, t16618: f64, t1704: f64, t18280: f64, t18281: f64, t18884: f64, t20236: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t395: f64, t4186: f64, t45: f64, t5028: f64, t51835: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t60754: f64, t6084: f64, t63202: f64, t63204: f64, t63206: f64, t6405: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t20400: f64, t3543: f64, t1765: f64, t57861: f64, t16784: f64, t5207: f64, t12248: f64, t3385: f64, t6439: f64, t3367: f64, t60717: f64, t1120: f64, t128: f64, t2435: f64, t6430: f64, t6422: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68006 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3561(t1000, t1076, t1079, t1096, t11210, t16312, t16313, t16343, t1696, t19380, t19381, t19428, t20178, t225, t3047, t3058, t3271, t342, t385, t53119, t53174, t6351, t64831, t64835, t64841, t64845, t64896, t64997, t65102, t65150, t65196, t65239, t65279, t67584, t67633, t67684, t67723, t67768, t67813, t67859, t67905, t67946, t67989, t995, t996);
        let t68038 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3562(t1078, t6258, t3057, t6343, t1076, t16284, t16292, t16295, t16302, t16312, t16313, t16327, t16352, t16371, t20188, t3060, t3066, t3269, t3325, t42052, t4764, t4773, t4778, t4941, t5016, t6392);
        let t68067 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3563(t1076, t16239, t16287, t16362, t1647, t1652, t19342, t19351, t20152, t20188, t3052, t3063, t3270, t3326, t42067, t42107, t43637, t43642, t4747, t4947, t53058, t53157, t55421, t6245, t6350);
        let t68097 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3564(t3046, t6343, t1000, t1073, t1076, t1096, t16243, t16333, t16340, t16344, t16352, t1652, t16600, t19856, t20151, t20204, t3067, t3269, t386, t4747, t4758, t4947, t5016, t53223, t55464, t65057);
        let t68130 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3565(t5015, t1076, t11120, t11214, t16249, t1652, t16603, t1696, t20191, t3058, t3066, t3076, t3261, t3269, t43656, t4758, t4778, t52994, t53281, t55461, t55475, t6235, t6245, t6251, t6350, t64912, t64989, t995, t996);
        let t68163 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3566(t20112, t342, t15669, t1678, t1076, t1079, t1097, t11121, t16152, t16255, t16275, t16333, t16597, t1696, t19429, t20219, t3058, t3060, t3063, t3270, t3325, t33754, t4752, t4941, t4947, t53015, t53034, t53174, t53180, t6244, t6392);
        let t68199 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3567(t1679, t994, t1071, t6235, t6343, t989, t1079, t1097, t11201, t11220, t16243, t16603, t16605, t19396, t20151, t20152, t20178, t20219, t3047, t3058, t3264, t3268, t3326, t4772, t4778, t4946, t6351, t65071, t65122, t995, t996, t999);
        let t68211 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3568(t20230, t3336, t1100, t1102, t198, t336, t5023, t64467, t64471, t64475, t64483, t64567, t64592, t64626, t64661, t64694, t64722, t64753, t64788, t64822, t65402, t65404, t65408, t65413, t65415, t65417, t65419, t65421, t68006, t68038, t68067, t68097, t68130, t68163, t68199);
        let t68231 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3569(t30, t265, t393, t63193, t63587, t63629, t63671, t63899, t63938, t64513, t64532, t68211, t1106, t13312, t1468, t1469, t15083, t16618, t1704, t18280, t18281, t18884, t20236, t2257, t2258, t2838, t3340, t395, t4186, t45, t5028, t51835, t5824, t5825, t605, t606, t60754, t6084, t63202, t63204, t63206, t6405, t895, dens_threshold, rho0, zeta_threshold);
        let (t68243, t68245, t68247, t68250, t68251, t68253) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3570(t20400, t3543, t1765, t57861, t16784, t5207, t12248, t3385, t6439, t3367, t60717, t1120, t128);
        let t68255 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3571(t2435, t6430);
        let t68257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3572(t2435, t6422);
    (t68231, t68243, t68245, t68247, t68250, t68251, t68253, t68255, t68257)
}
