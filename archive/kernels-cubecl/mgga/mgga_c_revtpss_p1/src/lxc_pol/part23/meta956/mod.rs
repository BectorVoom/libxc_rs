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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta956<F: Float>(t12809: F, t12916: F, t24839: F, t12787: F, t12866: F, t17448: F, t20770: F, t20838: F, t20933: F, t20937: F, t21008: F, t21017: F, t21046: F, t21310: F, t24232: F, t3625: F, t44517: F, t44521: F, t5405: F, t5407: F, t70014: F, t70819: F, t70917: F, t71047: F, t71061: F, t21063: F, t5362: F, t17308: F, t20846: F, t24639: F, t3172: F, t3711: F, t13062: F, t24545: F, t1122: F, t12855: F, t12910: F, t17736: F, t20858: F, t21119: F, t24713: F, t24751: F, t3626: F, t3720: F, t44704: F, t57147: F, t58824: F, t6688: F, t71055: F, t71117: F, t1261: F, t24807: F, t17377: F, t20786: F, t1042: F, t12956: F, t17505: F, t17547: F, t17550: F, t20811: F, t20825: F, t20864: F, t20868: F, t24640: F, t5302: F, t5381: F, t5391: F, t6625: F, t82368: F, t82543: F, t24604: F, t5384: F, t17794: F, t1794: F, t20946: F, t24248: F, t24612: F, t372: F, t58851: F, t58883: F, t58889: F, t71187: F, t71192: F, t71207: F, t71232: F, t17605: F, t21090: F, t127: F, t12988: F, t24617: F, t371: F, t20842: F, t5323: F, t12784: F, t17729: F, t21182: F, t24744: F, t24804: F, t44561: F, t44797: F, t5046: F, t59062: F, t6639: F, t71278: F, t71294: F, t71297: F, t82578: F, t21028: F, t1010: F, t22700: F, t1222: F, t1227: F, t17351: F, t17654: F, t17661: F, t17693: F, t17694: F, t17799: F, t20766: F, t20934: F, t21213: F, t21227: F, t5309: F, t5312: F, t57621: F, t57663: F, t71300: F, t81186: F, t82579: F, t83024: F, t21169: F, t5373: F, t21251: F, t17475: F, t5308: F, t59041: F, t71320: F, t71329: F, t71341: F, t81160: F, t81165: F, t81169: F, t81190: F, t81207: F, t17353: F, t20767: F, t20929: F, t3611: F, t44510: F, t44829: F, t5406: F, t57660: F, t69832: F, t71373: F, t71377: F, t71400: F, t71435: F, t71447: F, t71460: F, t83760: F, t1219: F, t24551: F, t21254: F, t21200: F, t21203: F, t44931: F, t59144: F, t59411: F, t71470: F, t71476: F, t71490: F, t71539: F, t71541: F, t5369: F, t59186: F, t71550: F, t71552: F, t71571: F, t71582: F, t71598: F, t71630: F, t71687: F, t71710: F, t71718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t83836 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3191::<F>(t12809, t12916, t24839, t12787, t12866, t17448, t20770, t20838, t20933, t20937, t21008, t21017, t21046, t21310, t24232, t3625, t44517, t44521, t5405, t5407, t70014, t70819, t70917, t71047, t71061);
        let t83865 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3192::<F>(t21063, t5362, t17308, t20846, t24639, t3172, t3711, t13062, t24545, t1122, t12855, t12910, t17736, t20858, t21119, t24713, t24751, t3626, t3720, t44704, t57147, t58824, t6688, t71055, t71117);
        let t83893 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3193::<F>(t1261, t24807, t3172, t17377, t20786, t1042, t12956, t17505, t17547, t17550, t20811, t20825, t20864, t20868, t24640, t3711, t5302, t5381, t5391, t6625, t82368, t82543);
        let t83915 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3194::<F>(t24604, t3172, t5384, t12866, t12956, t17794, t1794, t20946, t24248, t24612, t3625, t3626, t372, t5405, t58851, t58883, t58889, t71187, t71192, t71207, t71232);
        let t83938 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3195::<F>(t17605, t21090, t127, t12988, t24617, t371, t20842, t5323, t12784, t12787, t12866, t17729, t21182, t24744, t24804, t44561, t44797, t5046, t59062, t6639, t71278, t71294, t71297);
        let (t83943, t83950, t83973) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3196::<F>(t21119, t82578, t21028, t1010, t22700, t1222, t1227, t12866, t17351, t17654, t17661, t17693, t17694, t17799, t20766, t20770, t20934, t21213, t21227, t5309, t5312, t57621, t57663, t71300, t81186, t82579, t83024);
        let t83996 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3197::<F>(t21169, t5373, t21251, t1222, t17475, t5308, t5312, t59041, t71320, t71329, t71341, t81160, t81165, t81169, t81190, t81207);
        let t84020 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3198::<F>(t17351, t17353, t20766, t20767, t20929, t3611, t44510, t44521, t44829, t5406, t57660, t69832, t71061, t71373, t71377, t71400, t71435, t71447, t71460, t83760);
        let t84036 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3199::<F>(t1219, t24551, t21254, t5373, t20858, t21200, t21203, t44931, t59144, t59411, t71470, t71476, t71490, t71539, t71541);
        let t84049 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3200::<F>(t21213, t5369, t59186, t71550, t71552, t71571, t71582, t71598, t71630, t71687, t71710, t71718);
    (t83836, t83865, t83893, t83915, t83938, t83943, t83950, t83973, t83996, t84020, t84036, t84049)
}
