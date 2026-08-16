//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta780 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2781;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2782;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2783;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2784;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2785;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2786;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2787;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2788;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2789;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2790;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2791;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta780(t10760: f64, t40627: f64, t50613: f64, t14861: f64, t9794: f64, t10890: f64, t4458: f64, t10815: f64, t4426: f64, t40424: f64, t4430: f64, t14720: f64, t9775: f64, t1561: f64, t40360: f64, t14843: f64, t40864: f64, t10779: f64, t14931: f64, t1548: f64, t2724: f64, t10811: f64, t14693: f64, t40850: f64, t40851: f64, t40855: f64, t51074: f64, t51079: f64, t51081: f64, t51083: f64, t51086: f64, t2682: f64, t2719: f64, t4368: f64, t820: f64, t10778: f64, t221: f64, t10777: f64, t14792: f64, t2659: f64, t4503: f64, t816: f64, t14803: f64, t50769: f64, t4372: f64, t9784: f64, t2475: f64, t808: f64, t14787: f64, t50768: f64, t10627: f64, t10818: f64, t10872: f64, t14586: f64, t14691: f64, t14785: f64, t14791: f64, t14802: f64, t14894: f64, t1559: f64, t18632: f64, t2645: f64, t2745: f64, t2747: f64, t2749: f64, t2754: f64, t36833: f64, t40560: f64, t40862: f64, t40865: f64, t40868: f64, t4362: f64, t4364: f64, t4365: f64, t4450: f64, t50418: f64, t50474: f64, t800: f64, t836: f64, t50327: f64, t50365: f64, t50408: f64, t50480: f64, t50528: f64, t50558: f64, t50621: f64, t50675: f64, t50711: f64, t50750: f64, t50793: f64, t50979: f64, t51025: f64, t51072: f64, t14476: f64, t689: f64, t887: f64, t11028: f64, t1580: f64, t2439: f64, t10504: f64, t15002: f64, t9285: f64, t10505: f64, t137: f64, t41011: f64, t11015: f64, t4325: f64, t4477: f64, t9292: f64, t14472: f64, t213: f64, t225: f64, t257: f64, t2770: f64, t2828: f64, t41038: f64, t41043: f64, t41049: f64, t41052: f64, t41056: f64, t41058: f64, t4533: f64, t865: f64, t14979: f64, t779: f64, t11044: f64, t14983: f64, t14485: f64, t15014: f64, t9303: f64, t10510: f64, t14987: f64, t14991: f64, t41066: f64, t10982: f64, t1568: f64, t9646: f64, t252: f64, t2769: f64, t2782: f64, t886: f64, t10513: f64, t15011: f64, t15030: f64, t15038: f64, t2765: f64, t2772: f64, t41060: f64, t41063: f64, t41067: f64, t4487: f64, t4534: f64, t10995: f64, t11049: f64, t14990: f64, t14986: f64, t2453: f64, t10506: f64, t2458: f64, t4470: f64, t10069: f64, t14482: f64, t15003: f64, t41020: f64, t14939: f64, t786: f64, t867: f64, t2467: f64, t14567: f64, t10538: f64, t14662: f64, t251: f64, t40321: f64, t14502: f64, t14546: f64, t14972: f64, t2646: f64, t39612: f64, t39617: f64, t39622: f64, t4494: f64, t4504: f64, t4514: f64, t50666: f64, t50758: f64, t50916: f64, t837: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51089, t51093, t51096, t51099, t51100, t51102) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2781(t10760, t40627, t50613, t14861, t9794, t10890, t4458, t10815, t4426, t40424, t4430, t14720, t9775);
        let t51114 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2782(t1561, t40360, t14843, t40864, t10779, t14931, t1548, t2724, t10811, t14693, t40850, t40851, t40855, t51074, t51079, t51081, t51083, t51086, t51089, t51093, t51096, t51099, t51100, t51102);
        let (t51122, t51123, t51125, t51135) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2783(t2682, t2719, t4368, t820, t10778, t221, t10777, t14792, t2659, t4503, t816, t14803, t50769);
        let t51180 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2784(t14803, t14931, t51123, t4372, t9784, t2475, t808, t14787, t50768, t10627, t10818, t10872, t14586, t14691, t14785, t14791, t14802, t14894, t1548, t1559, t18632, t2645, t2724, t2745, t2747, t2749, t2754, t36833, t40560, t40862, t40865, t40868, t4362, t4364, t4365, t4450, t50418, t50474, t51122, t51125, t51135, t800, t836);
        let t51184 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2785(t50327, t50365, t50408, t50480, t50528, t50558, t50621, t50675, t50711, t50750, t50793, t50979, t51025, t51072, t51114, t51180);
        let (t51196, t51199, t51203, t51207) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2786(t14476, t689, t887, t11028, t1580, t2439, t10504, t15002, t9285, t10505, t137, t41011);
        let t51218 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2787(t51207, t11015, t4325, t4477, t9292, t14472, t2439, t887, t213, t225, t257, t2770, t2828, t41038, t41043, t41049, t41052, t41056, t41058, t4533, t51184, t51196, t51199, t51203, t865);
        let (t51227, t51231, t51234, t51237, t51240, t51241) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2788(t14979, t689, t779, t11044, t14983, t14485, t15014, t9303, t10510, t14987, t14991, t41066);
        let t51253 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2789(t10982, t1568, t9646, t252, t2769, t2782, t4533, t886, t10513, t15011, t15030, t15038, t2765, t2772, t41060, t41063, t41067, t4487, t4534, t51227, t51231, t51234, t51237, t51240, t51241);
        let (t51256, t51260, t51263, t51264, t51268) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2790(t10995, t11049, t14990, t14986, t2453, t10506, t2458, t4470, t10069, t14482, t15003, t41020);
        let (t51269, t51272, t51277, t51299, t51306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2791(t51268, t14939, t213, t4470, t786, t867, t2467, t14567, t2453, t10538, t14662, t251);
        let t51327 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2792(t213, t225, t40321, t10872, t14502, t14546, t14972, t2646, t39612, t39617, t39622, t4494, t4504, t4514, t50666, t50758, t50916, t51299, t51306, t820, t837, t879);
    (t51184, t51218, t51253, t51256, t51260, t51263, t51264, t51269, t51272, t51277, t51306, t51327)
}
