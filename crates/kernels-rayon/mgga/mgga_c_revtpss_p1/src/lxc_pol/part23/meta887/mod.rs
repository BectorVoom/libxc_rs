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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta887(t22590: f64, t625: f64, t22593: f64, t1513: f64, t5915: f64, t22629: f64, t1504: f64, t5823: f64, t22: f64, t39454: f64, t100: f64, t13475: f64, t2: f64, t21850: f64, t2255: f64, t22596: f64, t22597: f64, t22600: f64, t22604: f64, t22605: f64, t22608: f64, t2349: f64, t4269: f64, t4280: f64, t46196: f64, t49777: f64, t580: f64, t5895: f64, t5902: f64, t656: f64, t658: f64, t662: f64, t97: f64, t1509: f64, t5911: f64, t105: f64, t108: f64, t13496: f64, t1507: f64, t21861: f64, t21865: f64, t21869: f64, t21872: f64, t21873: f64, t22617: f64, t22624: f64, t2357: f64, t4279: f64, t4284: f64, t46212: f64, t49787: f64, t5907: f64, t661: f64, t75625: f64, t13458: f64, t21820: f64, t21876: f64, t22589: f64, t22628: f64, t2339: f64, t31035: f64, t4263: f64, t4287: f64, t46157: f64, t655: f64, t665: f64, t69: f64, t75542: f64, t114: f64, t46143: f64, t46144: f64, t49698: f64, t49701: f64, t49818: f64, t75526: f64, t75540: f64, t75639: f64, t75641: f64, t75643: f64, t116: f64, t22746: f64, t23384: f64, t689: f64, t779: f64, t14987: f64, t18797: f64, t23388: f64, t786: f64, t789: f64, t15011: f64, t39549: f64, t50155: f64, t50166: f64, t50178: f64, t6049: f64, t6072: f64, t61324: f64, t61330: f64, t61337: f64, t61344: f64, t61348: f64, t61351: f64, t23414: f64, t23413: f64, t41070: f64, t686: f64, t72: f64, t18805: f64, t50208: f64, t2765: f64, t39550: f64, t39554: f64, t39557: f64, t39558: f64, t50184: f64, t50187: f64, t50205: f64, t61355: f64, t61361: f64, t61367: f64, t61371: f64, t61378: f64, t4321: f64, t4481: f64, t63084: f64, t18323: f64, t23383: f64, t2770: f64, t40970: f64, t40978: f64, t50161: f64, t50214: f64, t50219: f64, t50221: f64, t50223: f64, t50240: f64, t61385: f64, t61397: f64, t61400: f64, t61403: f64, t61407: f64, t865: f64, t886: f64, t1580: f64, t18316: f64, t14480: f64, t252: f64, t2782: f64, t6071: f64, t11008: f64, t23404: f64, t40988: f64, t40998: f64, t4533: f64, t50236: f64, t50245: f64, t50248: f64, t50253: f64, t6048: f64, t61411: f64, t61419: f64, t61422: f64, t61430: f64, t61437: f64, t18800: f64, t41003: f64, t41004: f64, t41034: f64, t41037: f64, t41049: f64, t4487: f64, t51199: f64, t51203: f64, t51208: f64, t61441: f64, t61448: f64, t62516: f64, t62523: f64, t62528: f64, t2465: f64, t10995: f64, t23403: f64, t1579: f64, t18324: f64, t18784: f64, t41060: f64, t4474: f64, t51211: f64, t51213: f64, t51217: f64, t51234: f64, t51237: f64, t51240: f64, t51246: f64, t51260: f64, t51263: f64, t62549: f64, t62572: f64, t212: f64, t23359: f64, t780: f64, t23177: f64, t2798: f64, t14568: f64, t18730: f64, t14586: f64, t6016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75822, t75831, t75833, t75843, t75879, t75887) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2801(t22590, t625, t22593, t1513, t5915, t22629, t1504, t5823, t22, t39454, t100, t13475, t2, t21850, t2255, t22596, t22597, t22600, t22604, t22605, t22608, t2349, t4269, t4280, t46196, t49777, t580, t5895, t5902, t656, t658, t662, t97);
        let t75924 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2802(t1509, t5911, t105, t108, t13496, t1507, t2, t21861, t21865, t21869, t21872, t21873, t2255, t22617, t22624, t2357, t4279, t4284, t46212, t49787, t580, t5902, t5907, t661, t75625, t75879);
        let t75929 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2803(t13458, t21820, t21876, t22589, t22628, t2339, t31035, t4263, t4287, t46157, t5915, t655, t665, t69, t75542, t75822, t75831, t75833, t75843, t75887, t75924);
        let (t75931, t75941) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2804(t114, t46143, t46144, t49698, t49701, t49818, t75526, t75540, t75639, t75641, t75643, t75929, t116, t22746);
        let t75970 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2805(t23384, t689, t779, t14987, t18797, t23388, t786, t789, t15011, t39549, t50155, t50166, t50178, t6049, t6072, t61324, t61330, t61337, t61344, t61348, t61351);
        let t75990 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2806(t23414, t689, t779, t23413, t41070, t686, t72, t18805, t50208, t2765, t39550, t39554, t39557, t39558, t50184, t50187, t50205, t61355, t61361, t61367, t61371, t61378);
        let t76012 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2807(t4321, t6049, t689, t4481, t63084, t18323, t23383, t2770, t40970, t40978, t50161, t50214, t50219, t50221, t50223, t50240, t61385, t61397, t61400, t61403, t61407, t865, t886);
        let t76038 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2808(t1580, t18316, t689, t14480, t252, t2782, t6071, t11008, t23384, t23404, t2765, t40988, t40998, t4533, t50236, t50245, t50248, t50253, t6048, t61411, t61419, t61422, t61430, t61437, t865);
        let t76055 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2809(t4321, t6072, t689, t18800, t41003, t41004, t41034, t41037, t41049, t4487, t51199, t51203, t51208, t61441, t61448, t62516, t62523, t62528);
        let t76077 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2810(t23383, t2465, t686, t72, t10995, t23403, t1579, t18324, t18784, t2770, t41060, t4474, t51211, t51213, t51217, t51234, t51237, t51240, t51246, t51260, t51263, t62549, t62572, t865);
        let (t76081, t76100, t76104, t76106) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2811(t212, t23359, t689, t780, t23177, t2798, t686, t72, t14568, t18730, t14586, t6016);
    (t75931, t75941, t75970, t75990, t76012, t76038, t76055, t76077, t76081, t76100, t76104, t76106)
}
