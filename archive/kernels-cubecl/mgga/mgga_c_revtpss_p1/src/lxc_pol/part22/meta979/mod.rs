//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta979 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3290;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3291;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3292;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3293;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3294;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3295;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3296;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3297;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3298;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3299;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3300;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta979<F: Float>(t2475: F, t5962: F, t10696: F, t5966: F, t14468: F, t14643: F, t14649: F, t14653: F, t14656: F, t18392: F, t18586: F, t18592: F, t18599: F, t18600: F, t18603: F, t18608: F, t18609: F, t2394: F, t2430: F, t4415: F, t4416: F, t775: F, t833: F, t853: F, t231: F, t62347: F, t18616: F, t221: F, t2484: F, t2485: F, t10815: F, t5980: F, t40398: F, t6024: F, t18435: F, t10703: F, t2674: F, t10698: F, t10943: F, t14586: F, t14791: F, t14802: F, t18444: F, t23160: F, t2745: F, t4362: F, t4364: F, t50511: F, t50649: F, t51168: F, t51170: F, t6035: F, t825: F, t827: F, t828: F, t851: F, t14832: F, t2661: F, t10716: F, t18423: F, t14648: F, t4343: F, t18398: F, t2652: F, t18415: F, t9775: F, t18410: F, t2675: F, t18615: F, t243: F, t2662: F, t14923: F, t18478: F, t61519: F, t855: F, t10811: F, t18334: F, t18629: F, t10777: F, t10779: F, t14671: F, t18637: F, t50412: F, t14767: F, t14785: F, t14894: F, t1559: F, t18493: F, t18498: F, t36833: F, t50418: F, t50423: F, t50474: F, t50560: F, t51014: F, t51049: F, t51178: F, t837: F, t61471: F, t61544: F, t61578: F, t61599: F, t61622: F, t61657: F, t61694: F, t61730: F, t61748: F, t61772: F, t61789: F, t61814: F, t61829: F, t61852: F, t61879: F, t61899: F, t61929: F, t61954: F, t61987: F, t62008: F, t62039: F, t62074: F, t62101: F, t62123: F, t62158: F, t62186: F, t62199: F, t62231: F, t62258: F, t4321: F, t4534: F, t689: F, t213: F, t225: F, t257: F, t40994: F, t40998: F, t40999: F, t41003: F, t41004: F, t41006: F, t41014: F, t41021: F, t41029: F, t41034: F, t50245: F, t50248: F, t50253: F, t50259: F, t61430: F, t61437: F, t61441: F, t61448: F, t10995: F, t18312: F, t686: F, t72: F, t18804: F, t2470: F, t14489: F, t18324: F, t2765: F, t41037: F, t41038: F, t41049: F, t41052: F, t41056: F, t41060: F, t4474: F, t51196: F, t51199: F, t51203: F, t51207: F, t51211: F, t51213: F, t51216: F, t51227: F, t51231: F, t18657: F, t212: F, t780: F, t252: F, t2769: F, t2782: F, t6071: F, t886: F, t4500: F, t51421: F, t14495: F, t14567: F, t2798: F, t61532: F, t836: F, t39597: F, t6022: F, t10529: F, t10952: F, t18525: F, t2482: F, t5977: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t62351, t62361, t62383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3290::<F>(t2475, t5962, t10696, t5966, t14468, t14643, t14649, t14653, t14656, t18392, t18586, t18592, t18599, t18600, t18603, t18608, t18609, t2394, t2430, t4415, t4416, t775, t833, t853);
        let (t62385, t62392, t62399, t62401, t62403) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3291::<F>(t231, t62347, t62383, t18616, t221, t2484, t2485, t10815, t5980, t40398, t6024, t18435);
        let t62425 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3292::<F>(t10703, t2674, t62403, t10698, t10943, t14586, t14791, t14802, t18444, t23160, t2394, t2745, t4362, t4364, t50511, t50649, t51168, t51170, t5962, t6035, t62385, t62392, t62399, t62401, t825, t827, t828, t851);
        let (t62429, t62431, t62435, t62439, t62441, t62443) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3293::<F>(t14832, t2661, t62351, t775, t10716, t18423, t62361, t14648, t4343, t18398, t2652, t18415, t9775);
        let (t62445, t62453, t62458, t62460) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3294::<F>(t18410, t9775, t18392, t221, t2674, t2675, t18615, t231, t243, t2661, t2662, t14923, t18478);
        let t62462 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3295::<F>(t61519, t62429, t62431, t62435, t62439, t62441, t62443, t62445, t62453, t62458, t62460, t828, t851, t855);
        let t62504 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3296::<F>(t10811, t18334, t18629, t10777, t10779, t14671, t18637, t50412, t6035, t14586, t14767, t14785, t14791, t14894, t1559, t18493, t18498, t2745, t36833, t4362, t50418, t50423, t50474, t50560, t51014, t51049, t51178, t837);
        let t62509 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3297::<F>(t61471, t61544, t61578, t61599, t61622, t61657, t61694, t61730, t61748, t61772, t61789, t61814, t61829, t61852, t61879, t61899, t61929, t61954, t61987, t62008, t62039, t62074, t62101, t62123, t62158, t62186, t62199, t62231, t62258, t62425, t62462, t62504);
        let t62518 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3298::<F>(t4321, t4534, t689, t213, t225, t257, t40994, t40998, t40999, t41003, t41004, t41006, t41014, t41021, t41029, t41034, t50245, t50248, t50253, t50259, t61430, t61437, t61441, t61448, t62509);
        let t62545 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3299::<F>(t10995, t18312, t686, t72, t18804, t2470, t14489, t18324, t2765, t41037, t41038, t41049, t41052, t41056, t41060, t4474, t51196, t51199, t51203, t51207, t51211, t51213, t51216, t51227, t51231);
        let (t62549, t62572, t62577, t62583) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3300::<F>(t18657, t212, t689, t780, t252, t2769, t2782, t6071, t886, t4500, t51421, t14495, t14567);
        let (t62587, t62591, t62595, t62601) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3301::<F>(t18616, t2798, t686, t72, t61532, t836, t2782, t39597, t6022, t10529, t10952, t18525, t2482, t5977);
    (t62385, t62509, t62518, t62545, t62549, t62572, t62577, t62583, t62587, t62591, t62595, t62601)
}
