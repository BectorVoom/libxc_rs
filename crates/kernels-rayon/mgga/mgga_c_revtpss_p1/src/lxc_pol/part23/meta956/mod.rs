//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta956 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3191;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3192;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3193;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3194;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3195;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3196;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3197;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3198;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3199;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta956(t12809: f64, t12916: f64, t24839: f64, t12787: f64, t12866: f64, t17448: f64, t20770: f64, t20838: f64, t20933: f64, t20937: f64, t21008: f64, t21017: f64, t21046: f64, t21310: f64, t24232: f64, t3625: f64, t44517: f64, t44521: f64, t5405: f64, t5407: f64, t70014: f64, t70819: f64, t70917: f64, t71047: f64, t71061: f64, t21063: f64, t5362: f64, t17308: f64, t20846: f64, t24639: f64, t3172: f64, t3711: f64, t13062: f64, t24545: f64, t1122: f64, t12855: f64, t12910: f64, t17736: f64, t20858: f64, t21119: f64, t24713: f64, t24751: f64, t3626: f64, t3720: f64, t44704: f64, t57147: f64, t58824: f64, t6688: f64, t71055: f64, t71117: f64, t1261: f64, t24807: f64, t17377: f64, t20786: f64, t1042: f64, t12956: f64, t17505: f64, t17547: f64, t17550: f64, t20811: f64, t20825: f64, t20864: f64, t20868: f64, t24640: f64, t5302: f64, t5381: f64, t5391: f64, t6625: f64, t82368: f64, t82543: f64, t24604: f64, t5384: f64, t17794: f64, t1794: f64, t20946: f64, t24248: f64, t24612: f64, t372: f64, t58851: f64, t58883: f64, t58889: f64, t71187: f64, t71192: f64, t71207: f64, t71232: f64, t17605: f64, t21090: f64, t127: f64, t12988: f64, t24617: f64, t371: f64, t20842: f64, t5323: f64, t12784: f64, t17729: f64, t21182: f64, t24744: f64, t24804: f64, t44561: f64, t44797: f64, t5046: f64, t59062: f64, t6639: f64, t71278: f64, t71294: f64, t71297: f64, t82578: f64, t21028: f64, t1010: f64, t22700: f64, t1222: f64, t1227: f64, t17351: f64, t17654: f64, t17661: f64, t17693: f64, t17694: f64, t17799: f64, t20766: f64, t20934: f64, t21213: f64, t21227: f64, t5309: f64, t5312: f64, t57621: f64, t57663: f64, t71300: f64, t81186: f64, t82579: f64, t83024: f64, t21169: f64, t5373: f64, t21251: f64, t17475: f64, t5308: f64, t59041: f64, t71320: f64, t71329: f64, t71341: f64, t81160: f64, t81165: f64, t81169: f64, t81190: f64, t81207: f64, t17353: f64, t20767: f64, t20929: f64, t3611: f64, t44510: f64, t44829: f64, t5406: f64, t57660: f64, t69832: f64, t71373: f64, t71377: f64, t71400: f64, t71435: f64, t71447: f64, t71460: f64, t83760: f64, t1219: f64, t24551: f64, t21254: f64, t21200: f64, t21203: f64, t44931: f64, t59144: f64, t59411: f64, t71470: f64, t71476: f64, t71490: f64, t71539: f64, t71541: f64, t5369: f64, t59186: f64, t71550: f64, t71552: f64, t71571: f64, t71582: f64, t71598: f64, t71630: f64, t71687: f64, t71710: f64, t71718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t83836 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3191(t12809, t12916, t24839, t12787, t12866, t17448, t20770, t20838, t20933, t20937, t21008, t21017, t21046, t21310, t24232, t3625, t44517, t44521, t5405, t5407, t70014, t70819, t70917, t71047, t71061);
        let t83865 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3192(t21063, t5362, t17308, t20846, t24639, t3172, t3711, t13062, t24545, t1122, t12855, t12910, t17736, t20858, t21119, t24713, t24751, t3626, t3720, t44704, t57147, t58824, t6688, t71055, t71117);
        let t83893 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3193(t1261, t24807, t3172, t17377, t20786, t1042, t12956, t17505, t17547, t17550, t20811, t20825, t20864, t20868, t24640, t3711, t5302, t5381, t5391, t6625, t82368, t82543);
        let t83915 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3194(t24604, t3172, t5384, t12866, t12956, t17794, t1794, t20946, t24248, t24612, t3625, t3626, t372, t5405, t58851, t58883, t58889, t71187, t71192, t71207, t71232);
        let t83938 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3195(t17605, t21090, t127, t12988, t24617, t371, t20842, t5323, t12784, t12787, t12866, t17729, t21182, t24744, t24804, t44561, t44797, t5046, t59062, t6639, t71278, t71294, t71297);
        let (t83943, t83950, t83973) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3196(t21119, t82578, t21028, t1010, t22700, t1222, t1227, t12866, t17351, t17654, t17661, t17693, t17694, t17799, t20766, t20770, t20934, t21213, t21227, t5309, t5312, t57621, t57663, t71300, t81186, t82579, t83024);
        let t83996 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3197(t21169, t5373, t21251, t1222, t17475, t5308, t5312, t59041, t71320, t71329, t71341, t81160, t81165, t81169, t81190, t81207);
        let t84020 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3198(t17351, t17353, t20766, t20767, t20929, t3611, t44510, t44521, t44829, t5406, t57660, t69832, t71061, t71373, t71377, t71400, t71435, t71447, t71460, t83760);
        let t84036 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3199(t1219, t24551, t21254, t5373, t20858, t21200, t21203, t44931, t59144, t59411, t71470, t71476, t71490, t71539, t71541);
        let t84049 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3200(t21213, t5369, t59186, t71550, t71552, t71571, t71582, t71598, t71630, t71687, t71710, t71718);
    (t83836, t83865, t83893, t83915, t83938, t83943, t83950, t83973, t83996, t84020, t84036, t84049)
}
