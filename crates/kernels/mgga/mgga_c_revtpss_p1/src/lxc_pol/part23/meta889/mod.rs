//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta889 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2819;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2820;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2821;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2822;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2823;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2824;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2825;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2826;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2827;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2828;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2829;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta889<F: Float>(t4500: F, t62808: F, t23245: F, t2815: F, t39652: F, t39673: F, t4366: F, t4504: F, t51390: F, t51403: F, t51408: F, t62684: F, t62693: F, t62697: F, t76136: F, t820: F, t14546: F, t18525: F, t39697: F, t39701: F, t39719: F, t51424: F, t51430: F, t51435: F, t51445: F, t51452: F, t62716: F, t62723: F, t76131: F, t125: F, t23148: F, t23167: F, t23244: F, t1558: F, t5962: F, t10777: F, t14671: F, t14686: F, t6017: F, t10811: F, t23293: F, t1544: F, t23327: F, t23323: F, t14494: F, t14785: F, t14786: F, t14791: F, t14894: F, t18616: F, t18637: F, t2745: F, t2747: F, t2749: F, t36833: F, t40361: F, t4362: F, t4364: F, t4365: F, t4433: F, t50299: F, t50757: F, t5978: F, t61532: F, t76194: F, t837: F, t14586: F, t14931: F, t61715: F, t4423: F, t49886: F, t49887: F, t30: F, t33: F, zeta_threshold: F, t45: F, t14447: F, t1490: F, t18281: F, t18367: F, t19680: F, t22671: F, t22688: F, t2299: F, t4186: F, t4328: F, t5825: F, t606: F, t766: F, t80: F, t57: F, t14458: F, t1491: F, t18379: F, t2306: F, t4335: F, t770: F, t83: F, t221: F, t2674: F, t2675: F, t1559: F, t18426: F, t18444: F, t18632: F, t4424: F, t6016: F, t6022: F, t61538: F, t61540: F, t61542: F, t61550: F, t61560: F, t61564: F, t61568: F, t61570: F, t61791: F, t775: F, t828: F, t851: F, t855: F, t40409: F, t50370: F, t50372: F, t50375: F, t50377: F, t50381: F, t50383: F, t50385: F, t50387: F, t50390: F, t6035: F, t61572: F, t61574: F, t61576: F, t61582: F, t61612: F, t61616: F, t61749: F, t5966: F, t23160: F, t23279: F, t40425: F, t51014: F, t61620: F, t61623: F, t61628: F, t61630: F, t61632: F, t61641: F, t61645: F, t61660: F, t61669: F, t61673: F, t61675: F, t61677: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t76264 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2819::<F>(t4500, t62808, t23245, t2815, t39652, t39673, t4366, t4504, t51390, t51403, t51408, t62684, t62693, t62697, t76136, t820);
        let t76275 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2820::<F>(t14546, t18525, t39697, t39701, t39719, t51424, t51430, t51435, t51445, t51452, t62716, t62723, t76131);
        let (t76279, t76284) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2821::<F>(t125, t23148, t23167);
        let (t76289, t76302, t76313, t76315, t76321, t76330) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2822::<F>(t125, t23244, t1558, t5962, t10777, t14671, t14686, t6017, t10811, t23293, t1544, t23327);
        let t76343 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2823::<F>(t10811, t23323, t14494, t14785, t14786, t14791, t14894, t18616, t18637, t2745, t2747, t2749, t36833, t40361, t4362, t4364, t4365, t4366, t4433, t50299, t50757, t5978, t6017, t61532, t76194, t76279, t76284, t76289, t76302, t76313, t76315, t76321, t76330, t837);
        let (t76362, t76372, t76396) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2824::<F>(t14586, t14686, t14931, t61715, t1544, t4423, t49886, t49887);
        let t76397 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2825::<F>(t30, t33, t76396, zeta_threshold);
        let t76401 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2826::<F>(t45, t14447, t1490, t18281, t18367, t19680, t22671, t22688, t2299, t4186, t4328, t5825, t606, t76397, t766, t80, zeta_threshold);
        let t76419 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2827::<F>(t57, t14458, t1491, t18281, t18379, t19680, t22671, t22688, t2306, t4186, t4335, t5825, t606, t76397, t770, t83, zeta_threshold);
        let (t76421, t76434) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2828::<F>(t76401, t76419, t221, t23148, t2674, t2675, t14586, t14785, t14791, t1559, t18426, t18444, t18632, t2745, t4362, t4364, t4424, t4433, t6016, t6022, t61538, t61540, t61542, t61550, t61560, t61564, t61568, t61570, t61791, t76284, t76362, t76372, t775, t828, t837, t851, t855);
        let t76458 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2829::<F>(t14791, t2745, t40409, t50370, t50372, t50375, t50377, t50381, t50383, t50385, t50387, t50390, t6035, t61572, t61574, t61576, t61582, t61612, t61616, t61749, t76302, t837);
        let (t76474, t76493) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2830::<F>(t1558, t5966, t14785, t14786, t14791, t23160, t23279, t2745, t2749, t40425, t4362, t51014, t6022, t61620, t61623, t61628, t61630, t61632, t61641, t61645, t61660, t61669, t61673, t61675, t61677, t76302, t837);
    (t76264, t76275, t76284, t76343, t76372, t76396, t76397, t76421, t76434, t76458, t76474, t76493)
}
