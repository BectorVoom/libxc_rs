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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3244;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3245;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3246;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3247;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3248;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3249;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3250;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta971<F: Float>(t14923: F, t18634: F, t10726: F, t18408: F, t2661: F, t4366: F, t18608: F, t2662: F, t837: F, t18632: F, t4352: F, t10815: F, t6019: F, t10845: F, t18531: F, t18618: F, t2741: F, t18622: F, t14785: F, t18627: F, t2745: F, t2747: F, t2749: F, t2754: F, t50351: F, t5962: F, t836: F, t6016: F, t853: F, t18392: F, t2477: F, t40374: F, t40393: F, t40395: F, t40399: F, t40409: F, t40411: F, t50353: F, t50370: F, t50372: F, t50374: F, t775: F, t828: F, t851: F, t14718: F, t18637: F, t50583: F, t6035: F, t50377: F, t50381: F, t50383: F, t50385: F, t50387: F, t50389: F, t50394: F, t50399: F, t18432: F, t40336: F, t5977: F, t10786: F, t18495: F, t2652: F, t18500: F, t18493: F, t221: F, t2674: F, t40683: F, t18441: F, t9775: F, t4423: F, t231: F, t10698: F, t10770: F, t18469: F, t2430: F, t40425: F, t50409: F, t5966: F, t825: F, t827: F, t18437: F, t4424: F, t18413: F, t10716: F, t18402: F, t10722: F, t5993: F, t2722: F, t40325: F, t18481: F, t50768: F, t51176: F, t18333: F, t50769: F, t14547: F, t14894: F, t18426: F, t4364: F, t50415: F, t50757: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61550, t61560, t61564, t61568, t61570) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3244::<F>(t14923, t18634, t10726, t18408, t2661, t4366, t18608, t2662, t837, t18632, t4352, t10815, t6019);
        let t61578 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3245::<F>(t10845, t18531, t18618, t2741, t18622, t14785, t18627, t2745, t2747, t2749, t2754, t50351, t5962, t61550, t61560, t61564, t61568, t61570, t836);
        let t61599 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3246::<F>(t6016, t853, t2661, t2662, t2749, t18392, t2477, t40374, t40393, t40395, t40399, t40409, t40411, t50353, t50370, t50372, t50374, t775, t828, t851);
        let t61622 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3247::<F>(t14718, t18637, t2661, t2662, t50583, t6035, t18408, t837, t50377, t50381, t50383, t50385, t50387, t50389, t50394, t50399);
        let (t61623, t61625, t61628, t61630, t61632, t61639) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3248::<F>(t18432, t40336, t5977, t853, t10726, t10786, t2661, t18495, t2652, t18500, t18493, t221);
        let (t61647, t61648, t61657) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3249::<F>(t2674, t40683, t61639, t18441, t9775, t4423, t231, t10698, t10770, t18469, t2430, t2745, t2754, t40425, t50409, t5966, t61623, t61628, t61630, t61632, t825, t827, t828, t851);
        let (t61660, t61669, t61673, t61675, t61677) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3250::<F>(t18437, t2652, t2661, t2662, t4352, t4424, t18413, t837, t10716, t18402, t10722, t5993);
        let (t61679, t61694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3251::<F>(t2722, t40325, t18481, t50768, t51176, t18333, t50769, t14547, t14894, t18426, t2430, t2477, t4364, t50415, t50757, t5962, t61660, t61669, t61673, t61675, t61677, t828, t851);
    (t61578, t61599, t61622, t61625, t61647, t61648, t61657, t61679, t61694)
}
