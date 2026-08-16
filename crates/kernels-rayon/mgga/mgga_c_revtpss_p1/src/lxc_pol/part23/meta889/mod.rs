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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta889(t4500: f64, t62808: f64, t23245: f64, t2815: f64, t39652: f64, t39673: f64, t4366: f64, t4504: f64, t51390: f64, t51403: f64, t51408: f64, t62684: f64, t62693: f64, t62697: f64, t76136: f64, t820: f64, t14546: f64, t18525: f64, t39697: f64, t39701: f64, t39719: f64, t51424: f64, t51430: f64, t51435: f64, t51445: f64, t51452: f64, t62716: f64, t62723: f64, t76131: f64, t125: f64, t23148: f64, t23167: f64, t23244: f64, t1558: f64, t5962: f64, t10777: f64, t14671: f64, t14686: f64, t6017: f64, t10811: f64, t23293: f64, t1544: f64, t23327: f64, t23323: f64, t14494: f64, t14785: f64, t14786: f64, t14791: f64, t14894: f64, t18616: f64, t18637: f64, t2745: f64, t2747: f64, t2749: f64, t36833: f64, t40361: f64, t4362: f64, t4364: f64, t4365: f64, t4433: f64, t50299: f64, t50757: f64, t5978: f64, t61532: f64, t76194: f64, t837: f64, t14586: f64, t14931: f64, t61715: f64, t4423: f64, t49886: f64, t49887: f64, t30: f64, t33: f64, zeta_threshold: f64, t45: f64, t14447: f64, t1490: f64, t18281: f64, t18367: f64, t19680: f64, t22671: f64, t22688: f64, t2299: f64, t4186: f64, t4328: f64, t5825: f64, t606: f64, t766: f64, t80: f64, t57: f64, t14458: f64, t1491: f64, t18379: f64, t2306: f64, t4335: f64, t770: f64, t83: f64, t221: f64, t2674: f64, t2675: f64, t1559: f64, t18426: f64, t18444: f64, t18632: f64, t4424: f64, t6016: f64, t6022: f64, t61538: f64, t61540: f64, t61542: f64, t61550: f64, t61560: f64, t61564: f64, t61568: f64, t61570: f64, t61791: f64, t775: f64, t828: f64, t851: f64, t855: f64, t40409: f64, t50370: f64, t50372: f64, t50375: f64, t50377: f64, t50381: f64, t50383: f64, t50385: f64, t50387: f64, t50390: f64, t6035: f64, t61572: f64, t61574: f64, t61576: f64, t61582: f64, t61612: f64, t61616: f64, t61749: f64, t5966: f64, t23160: f64, t23279: f64, t40425: f64, t51014: f64, t61620: f64, t61623: f64, t61628: f64, t61630: f64, t61632: f64, t61641: f64, t61645: f64, t61660: f64, t61669: f64, t61673: f64, t61675: f64, t61677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t76264 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2819(t4500, t62808, t23245, t2815, t39652, t39673, t4366, t4504, t51390, t51403, t51408, t62684, t62693, t62697, t76136, t820);
        let t76275 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2820(t14546, t18525, t39697, t39701, t39719, t51424, t51430, t51435, t51445, t51452, t62716, t62723, t76131);
        let (t76279, t76284) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2821(t125, t23148, t23167);
        let (t76289, t76302, t76313, t76315, t76321, t76330) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2822(t125, t23244, t1558, t5962, t10777, t14671, t14686, t6017, t10811, t23293, t1544, t23327);
        let t76343 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2823(t10811, t23323, t14494, t14785, t14786, t14791, t14894, t18616, t18637, t2745, t2747, t2749, t36833, t40361, t4362, t4364, t4365, t4366, t4433, t50299, t50757, t5978, t6017, t61532, t76194, t76279, t76284, t76289, t76302, t76313, t76315, t76321, t76330, t837);
        let (t76362, t76372, t76396) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2824(t14586, t14686, t14931, t61715, t1544, t4423, t49886, t49887);
        let t76397 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2825(t30, t33, t76396, zeta_threshold);
        let t76401 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2826(t45, t14447, t1490, t18281, t18367, t19680, t22671, t22688, t2299, t4186, t4328, t5825, t606, t76397, t766, t80, zeta_threshold);
        let t76419 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2827(t57, t14458, t1491, t18281, t18379, t19680, t22671, t22688, t2306, t4186, t4335, t5825, t606, t76397, t770, t83, zeta_threshold);
        let (t76421, t76434) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2828(t76401, t76419, t221, t23148, t2674, t2675, t14586, t14785, t14791, t1559, t18426, t18444, t18632, t2745, t4362, t4364, t4424, t4433, t6016, t6022, t61538, t61540, t61542, t61550, t61560, t61564, t61568, t61570, t61791, t76284, t76362, t76372, t775, t828, t837, t851, t855);
        let t76458 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2829(t14791, t2745, t40409, t50370, t50372, t50375, t50377, t50381, t50383, t50385, t50387, t50390, t6035, t61572, t61574, t61576, t61582, t61612, t61616, t61749, t76302, t837);
        let (t76474, t76493) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2830(t1558, t5966, t14785, t14786, t14791, t23160, t23279, t2745, t2749, t40425, t4362, t51014, t6022, t61620, t61623, t61628, t61630, t61632, t61641, t61645, t61660, t61669, t61673, t61675, t61677, t76302, t837);
    (t76264, t76275, t76284, t76343, t76372, t76396, t76397, t76421, t76434, t76458, t76474, t76493)
}
