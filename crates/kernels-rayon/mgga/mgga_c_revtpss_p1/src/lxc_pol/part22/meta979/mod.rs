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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta979(t2475: f64, t5962: f64, t10696: f64, t5966: f64, t14468: f64, t14643: f64, t14649: f64, t14653: f64, t14656: f64, t18392: f64, t18586: f64, t18592: f64, t18599: f64, t18600: f64, t18603: f64, t18608: f64, t18609: f64, t2394: f64, t2430: f64, t4415: f64, t4416: f64, t775: f64, t833: f64, t853: f64, t231: f64, t62347: f64, t18616: f64, t221: f64, t2484: f64, t2485: f64, t10815: f64, t5980: f64, t40398: f64, t6024: f64, t18435: f64, t10703: f64, t2674: f64, t10698: f64, t10943: f64, t14586: f64, t14791: f64, t14802: f64, t18444: f64, t23160: f64, t2745: f64, t4362: f64, t4364: f64, t50511: f64, t50649: f64, t51168: f64, t51170: f64, t6035: f64, t825: f64, t827: f64, t828: f64, t851: f64, t14832: f64, t2661: f64, t10716: f64, t18423: f64, t14648: f64, t4343: f64, t18398: f64, t2652: f64, t18415: f64, t9775: f64, t18410: f64, t2675: f64, t18615: f64, t243: f64, t2662: f64, t14923: f64, t18478: f64, t61519: f64, t855: f64, t10811: f64, t18334: f64, t18629: f64, t10777: f64, t10779: f64, t14671: f64, t18637: f64, t50412: f64, t14767: f64, t14785: f64, t14894: f64, t1559: f64, t18493: f64, t18498: f64, t36833: f64, t50418: f64, t50423: f64, t50474: f64, t50560: f64, t51014: f64, t51049: f64, t51178: f64, t837: f64, t61471: f64, t61544: f64, t61578: f64, t61599: f64, t61622: f64, t61657: f64, t61694: f64, t61730: f64, t61748: f64, t61772: f64, t61789: f64, t61814: f64, t61829: f64, t61852: f64, t61879: f64, t61899: f64, t61929: f64, t61954: f64, t61987: f64, t62008: f64, t62039: f64, t62074: f64, t62101: f64, t62123: f64, t62158: f64, t62186: f64, t62199: f64, t62231: f64, t62258: f64, t4321: f64, t4534: f64, t689: f64, t213: f64, t225: f64, t257: f64, t40994: f64, t40998: f64, t40999: f64, t41003: f64, t41004: f64, t41006: f64, t41014: f64, t41021: f64, t41029: f64, t41034: f64, t50245: f64, t50248: f64, t50253: f64, t50259: f64, t61430: f64, t61437: f64, t61441: f64, t61448: f64, t10995: f64, t18312: f64, t686: f64, t72: f64, t18804: f64, t2470: f64, t14489: f64, t18324: f64, t2765: f64, t41037: f64, t41038: f64, t41049: f64, t41052: f64, t41056: f64, t41060: f64, t4474: f64, t51196: f64, t51199: f64, t51203: f64, t51207: f64, t51211: f64, t51213: f64, t51216: f64, t51227: f64, t51231: f64, t18657: f64, t212: f64, t780: f64, t252: f64, t2769: f64, t2782: f64, t6071: f64, t886: f64, t4500: f64, t51421: f64, t14495: f64, t14567: f64, t2798: f64, t61532: f64, t836: f64, t39597: f64, t6022: f64, t10529: f64, t10952: f64, t18525: f64, t2482: f64, t5977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62351, t62361, t62383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3290(t2475, t5962, t10696, t5966, t14468, t14643, t14649, t14653, t14656, t18392, t18586, t18592, t18599, t18600, t18603, t18608, t18609, t2394, t2430, t4415, t4416, t775, t833, t853);
        let (t62385, t62392, t62399, t62401, t62403) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3291(t231, t62347, t62383, t18616, t221, t2484, t2485, t10815, t5980, t40398, t6024, t18435);
        let t62425 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3292(t10703, t2674, t62403, t10698, t10943, t14586, t14791, t14802, t18444, t23160, t2394, t2745, t4362, t4364, t50511, t50649, t51168, t51170, t5962, t6035, t62385, t62392, t62399, t62401, t825, t827, t828, t851);
        let (t62429, t62431, t62435, t62439, t62441, t62443) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3293(t14832, t2661, t62351, t775, t10716, t18423, t62361, t14648, t4343, t18398, t2652, t18415, t9775);
        let (t62445, t62453, t62458, t62460) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3294(t18410, t9775, t18392, t221, t2674, t2675, t18615, t231, t243, t2661, t2662, t14923, t18478);
        let t62462 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3295(t61519, t62429, t62431, t62435, t62439, t62441, t62443, t62445, t62453, t62458, t62460, t828, t851, t855);
        let t62504 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3296(t10811, t18334, t18629, t10777, t10779, t14671, t18637, t50412, t6035, t14586, t14767, t14785, t14791, t14894, t1559, t18493, t18498, t2745, t36833, t4362, t50418, t50423, t50474, t50560, t51014, t51049, t51178, t837);
        let t62509 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3297(t61471, t61544, t61578, t61599, t61622, t61657, t61694, t61730, t61748, t61772, t61789, t61814, t61829, t61852, t61879, t61899, t61929, t61954, t61987, t62008, t62039, t62074, t62101, t62123, t62158, t62186, t62199, t62231, t62258, t62425, t62462, t62504);
        let t62518 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3298(t4321, t4534, t689, t213, t225, t257, t40994, t40998, t40999, t41003, t41004, t41006, t41014, t41021, t41029, t41034, t50245, t50248, t50253, t50259, t61430, t61437, t61441, t61448, t62509);
        let t62545 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3299(t10995, t18312, t686, t72, t18804, t2470, t14489, t18324, t2765, t41037, t41038, t41049, t41052, t41056, t41060, t4474, t51196, t51199, t51203, t51207, t51211, t51213, t51216, t51227, t51231);
        let (t62549, t62572, t62577, t62583) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3300(t18657, t212, t689, t780, t252, t2769, t2782, t6071, t886, t4500, t51421, t14495, t14567);
        let (t62587, t62591, t62595, t62601) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3301(t18616, t2798, t686, t72, t61532, t836, t2782, t39597, t6022, t10529, t10952, t18525, t2482, t5977);
    (t62385, t62509, t62518, t62545, t62549, t62572, t62577, t62583, t62587, t62591, t62595, t62601)
}
