//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta887 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2801;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2802;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2803;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2804;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2805;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2806;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2807;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2808;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2809;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2810;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta887<F: Float>(t22590: F, t625: F, t22593: F, t1513: F, t5915: F, t22629: F, t1504: F, t5823: F, t22: F, t39454: F, t100: F, t13475: F, t2: F, t21850: F, t2255: F, t22596: F, t22597: F, t22600: F, t22604: F, t22605: F, t22608: F, t2349: F, t4269: F, t4280: F, t46196: F, t49777: F, t580: F, t5895: F, t5902: F, t656: F, t658: F, t662: F, t97: F, t1509: F, t5911: F, t105: F, t108: F, t13496: F, t1507: F, t21861: F, t21865: F, t21869: F, t21872: F, t21873: F, t22617: F, t22624: F, t2357: F, t4279: F, t4284: F, t46212: F, t49787: F, t5907: F, t661: F, t75625: F, t13458: F, t21820: F, t21876: F, t22589: F, t22628: F, t2339: F, t31035: F, t4263: F, t4287: F, t46157: F, t655: F, t665: F, t69: F, t75542: F, t114: F, t46143: F, t46144: F, t49698: F, t49701: F, t49818: F, t75526: F, t75540: F, t75639: F, t75641: F, t75643: F, t116: F, t22746: F, t23384: F, t689: F, t779: F, t14987: F, t18797: F, t23388: F, t786: F, t789: F, t15011: F, t39549: F, t50155: F, t50166: F, t50178: F, t6049: F, t6072: F, t61324: F, t61330: F, t61337: F, t61344: F, t61348: F, t61351: F, t23414: F, t23413: F, t41070: F, t686: F, t72: F, t18805: F, t50208: F, t2765: F, t39550: F, t39554: F, t39557: F, t39558: F, t50184: F, t50187: F, t50205: F, t61355: F, t61361: F, t61367: F, t61371: F, t61378: F, t4321: F, t4481: F, t63084: F, t18323: F, t23383: F, t2770: F, t40970: F, t40978: F, t50161: F, t50214: F, t50219: F, t50221: F, t50223: F, t50240: F, t61385: F, t61397: F, t61400: F, t61403: F, t61407: F, t865: F, t886: F, t1580: F, t18316: F, t14480: F, t252: F, t2782: F, t6071: F, t11008: F, t23404: F, t40988: F, t40998: F, t4533: F, t50236: F, t50245: F, t50248: F, t50253: F, t6048: F, t61411: F, t61419: F, t61422: F, t61430: F, t61437: F, t18800: F, t41003: F, t41004: F, t41034: F, t41037: F, t41049: F, t4487: F, t51199: F, t51203: F, t51208: F, t61441: F, t61448: F, t62516: F, t62523: F, t62528: F, t2465: F, t10995: F, t23403: F, t1579: F, t18324: F, t18784: F, t41060: F, t4474: F, t51211: F, t51213: F, t51217: F, t51234: F, t51237: F, t51240: F, t51246: F, t51260: F, t51263: F, t62549: F, t62572: F, t212: F, t23359: F, t780: F, t23177: F, t2798: F, t14568: F, t18730: F, t14586: F, t6016: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t75822, t75831, t75833, t75843, t75879, t75887) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2801::<F>(t22590, t625, t22593, t1513, t5915, t22629, t1504, t5823, t22, t39454, t100, t13475, t2, t21850, t2255, t22596, t22597, t22600, t22604, t22605, t22608, t2349, t4269, t4280, t46196, t49777, t580, t5895, t5902, t656, t658, t662, t97);
        let t75924 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2802::<F>(t1509, t5911, t105, t108, t13496, t1507, t2, t21861, t21865, t21869, t21872, t21873, t2255, t22617, t22624, t2357, t4279, t4284, t46212, t49787, t580, t5902, t5907, t661, t75625, t75879);
        let t75929 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2803::<F>(t13458, t21820, t21876, t22589, t22628, t2339, t31035, t4263, t4287, t46157, t5915, t655, t665, t69, t75542, t75822, t75831, t75833, t75843, t75887, t75924);
        let (t75931, t75941) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2804::<F>(t114, t46143, t46144, t49698, t49701, t49818, t75526, t75540, t75639, t75641, t75643, t75929, t116, t22746);
        let t75970 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2805::<F>(t23384, t689, t779, t14987, t18797, t23388, t786, t789, t15011, t39549, t50155, t50166, t50178, t6049, t6072, t61324, t61330, t61337, t61344, t61348, t61351);
        let t75990 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2806::<F>(t23414, t689, t779, t23413, t41070, t686, t72, t18805, t50208, t2765, t39550, t39554, t39557, t39558, t50184, t50187, t50205, t61355, t61361, t61367, t61371, t61378);
        let t76012 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2807::<F>(t4321, t6049, t689, t4481, t63084, t18323, t23383, t2770, t40970, t40978, t50161, t50214, t50219, t50221, t50223, t50240, t61385, t61397, t61400, t61403, t61407, t865, t886);
        let t76038 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2808::<F>(t1580, t18316, t689, t14480, t252, t2782, t6071, t11008, t23384, t23404, t2765, t40988, t40998, t4533, t50236, t50245, t50248, t50253, t6048, t61411, t61419, t61422, t61430, t61437, t865);
        let t76055 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2809::<F>(t4321, t6072, t689, t18800, t41003, t41004, t41034, t41037, t41049, t4487, t51199, t51203, t51208, t61441, t61448, t62516, t62523, t62528);
        let t76077 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2810::<F>(t23383, t2465, t686, t72, t10995, t23403, t1579, t18324, t18784, t2770, t41060, t4474, t51211, t51213, t51217, t51234, t51237, t51240, t51246, t51260, t51263, t62549, t62572, t865);
        let (t76081, t76100, t76104, t76106) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2811::<F>(t212, t23359, t689, t780, t23177, t2798, t686, t72, t14568, t18730, t14586, t6016);
    (t75931, t75941, t75970, t75990, t76012, t76038, t76055, t76077, t76081, t76100, t76104, t76106)
}
