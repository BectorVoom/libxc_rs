//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta971 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3244;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3245;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3246;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3247;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3248;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3249;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3250;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta971(t14923: f64, t18634: f64, t10726: f64, t18408: f64, t2661: f64, t4366: f64, t18608: f64, t2662: f64, t837: f64, t18632: f64, t4352: f64, t10815: f64, t6019: f64, t10845: f64, t18531: f64, t18618: f64, t2741: f64, t18622: f64, t14785: f64, t18627: f64, t2745: f64, t2747: f64, t2749: f64, t2754: f64, t50351: f64, t5962: f64, t836: f64, t6016: f64, t853: f64, t18392: f64, t2477: f64, t40374: f64, t40393: f64, t40395: f64, t40399: f64, t40409: f64, t40411: f64, t50353: f64, t50370: f64, t50372: f64, t50374: f64, t775: f64, t828: f64, t851: f64, t14718: f64, t18637: f64, t50583: f64, t6035: f64, t50377: f64, t50381: f64, t50383: f64, t50385: f64, t50387: f64, t50389: f64, t50394: f64, t50399: f64, t18432: f64, t40336: f64, t5977: f64, t10786: f64, t18495: f64, t2652: f64, t18500: f64, t18493: f64, t221: f64, t2674: f64, t40683: f64, t18441: f64, t9775: f64, t4423: f64, t231: f64, t10698: f64, t10770: f64, t18469: f64, t2430: f64, t40425: f64, t50409: f64, t5966: f64, t825: f64, t827: f64, t18437: f64, t4424: f64, t18413: f64, t10716: f64, t18402: f64, t10722: f64, t5993: f64, t2722: f64, t40325: f64, t18481: f64, t50768: f64, t51176: f64, t18333: f64, t50769: f64, t14547: f64, t14894: f64, t18426: f64, t4364: f64, t50415: f64, t50757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61550, t61560, t61564, t61568, t61570) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3244(t14923, t18634, t10726, t18408, t2661, t4366, t18608, t2662, t837, t18632, t4352, t10815, t6019);
        let t61578 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3245(t10845, t18531, t18618, t2741, t18622, t14785, t18627, t2745, t2747, t2749, t2754, t50351, t5962, t61550, t61560, t61564, t61568, t61570, t836);
        let t61599 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3246(t6016, t853, t2661, t2662, t2749, t18392, t2477, t40374, t40393, t40395, t40399, t40409, t40411, t50353, t50370, t50372, t50374, t775, t828, t851);
        let t61622 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3247(t14718, t18637, t2661, t2662, t50583, t6035, t18408, t837, t50377, t50381, t50383, t50385, t50387, t50389, t50394, t50399);
        let (t61623, t61625, t61628, t61630, t61632, t61639) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3248(t18432, t40336, t5977, t853, t10726, t10786, t2661, t18495, t2652, t18500, t18493, t221);
        let (t61647, t61648, t61657) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3249(t2674, t40683, t61639, t18441, t9775, t4423, t231, t10698, t10770, t18469, t2430, t2745, t2754, t40425, t50409, t5966, t61623, t61628, t61630, t61632, t825, t827, t828, t851);
        let (t61660, t61669, t61673, t61675, t61677) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3250(t18437, t2652, t2661, t2662, t4352, t4424, t18413, t837, t10716, t18402, t10722, t5993);
        let (t61679, t61694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3251(t2722, t40325, t18481, t50768, t51176, t18333, t50769, t14547, t14894, t18426, t2430, t2477, t4364, t50415, t50757, t5962, t61660, t61669, t61673, t61675, t61677, t828, t851);
    (t61578, t61599, t61622, t61625, t61647, t61648, t61657, t61679, t61694)
}
