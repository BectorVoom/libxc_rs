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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta780<F: Float>(t10760: F, t40627: F, t50613: F, t14861: F, t9794: F, t10890: F, t4458: F, t10815: F, t4426: F, t40424: F, t4430: F, t14720: F, t9775: F, t1561: F, t40360: F, t14843: F, t40864: F, t10779: F, t14931: F, t1548: F, t2724: F, t10811: F, t14693: F, t40850: F, t40851: F, t40855: F, t51074: F, t51079: F, t51081: F, t51083: F, t51086: F, t2682: F, t2719: F, t4368: F, t820: F, t10778: F, t221: F, t10777: F, t14792: F, t2659: F, t4503: F, t816: F, t14803: F, t50769: F, t4372: F, t9784: F, t2475: F, t808: F, t14787: F, t50768: F, t10627: F, t10818: F, t10872: F, t14586: F, t14691: F, t14785: F, t14791: F, t14802: F, t14894: F, t1559: F, t18632: F, t2645: F, t2745: F, t2747: F, t2749: F, t2754: F, t36833: F, t40560: F, t40862: F, t40865: F, t40868: F, t4362: F, t4364: F, t4365: F, t4450: F, t50418: F, t50474: F, t800: F, t836: F, t50327: F, t50365: F, t50408: F, t50480: F, t50528: F, t50558: F, t50621: F, t50675: F, t50711: F, t50750: F, t50793: F, t50979: F, t51025: F, t51072: F, t14476: F, t689: F, t887: F, t11028: F, t1580: F, t2439: F, t10504: F, t15002: F, t9285: F, t10505: F, t137: F, t41011: F, t11015: F, t4325: F, t4477: F, t9292: F, t14472: F, t213: F, t225: F, t257: F, t2770: F, t2828: F, t41038: F, t41043: F, t41049: F, t41052: F, t41056: F, t41058: F, t4533: F, t865: F, t14979: F, t779: F, t11044: F, t14983: F, t14485: F, t15014: F, t9303: F, t10510: F, t14987: F, t14991: F, t41066: F, t10982: F, t1568: F, t9646: F, t252: F, t2769: F, t2782: F, t886: F, t10513: F, t15011: F, t15030: F, t15038: F, t2765: F, t2772: F, t41060: F, t41063: F, t41067: F, t4487: F, t4534: F, t10995: F, t11049: F, t14990: F, t14986: F, t2453: F, t10506: F, t2458: F, t4470: F, t10069: F, t14482: F, t15003: F, t41020: F, t14939: F, t786: F, t867: F, t2467: F, t14567: F, t10538: F, t14662: F, t251: F, t40321: F, t14502: F, t14546: F, t14972: F, t2646: F, t39612: F, t39617: F, t39622: F, t4494: F, t4504: F, t4514: F, t50666: F, t50758: F, t50916: F, t837: F, t879: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51089, t51093, t51096, t51099, t51100, t51102) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2781::<F>(t10760, t40627, t50613, t14861, t9794, t10890, t4458, t10815, t4426, t40424, t4430, t14720, t9775);
        let t51114 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2782::<F>(t1561, t40360, t14843, t40864, t10779, t14931, t1548, t2724, t10811, t14693, t40850, t40851, t40855, t51074, t51079, t51081, t51083, t51086, t51089, t51093, t51096, t51099, t51100, t51102);
        let (t51122, t51123, t51125, t51135) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2783::<F>(t2682, t2719, t4368, t820, t10778, t221, t10777, t14792, t2659, t4503, t816, t14803, t50769);
        let t51180 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2784::<F>(t14803, t14931, t51123, t4372, t9784, t2475, t808, t14787, t50768, t10627, t10818, t10872, t14586, t14691, t14785, t14791, t14802, t14894, t1548, t1559, t18632, t2645, t2724, t2745, t2747, t2749, t2754, t36833, t40560, t40862, t40865, t40868, t4362, t4364, t4365, t4450, t50418, t50474, t51122, t51125, t51135, t800, t836);
        let t51184 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2785::<F>(t50327, t50365, t50408, t50480, t50528, t50558, t50621, t50675, t50711, t50750, t50793, t50979, t51025, t51072, t51114, t51180);
        let (t51196, t51199, t51203, t51207) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2786::<F>(t14476, t689, t887, t11028, t1580, t2439, t10504, t15002, t9285, t10505, t137, t41011);
        let t51218 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2787::<F>(t51207, t11015, t4325, t4477, t9292, t14472, t2439, t887, t213, t225, t257, t2770, t2828, t41038, t41043, t41049, t41052, t41056, t41058, t4533, t51184, t51196, t51199, t51203, t865);
        let (t51227, t51231, t51234, t51237, t51240, t51241) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2788::<F>(t14979, t689, t779, t11044, t14983, t14485, t15014, t9303, t10510, t14987, t14991, t41066);
        let t51253 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2789::<F>(t10982, t1568, t9646, t252, t2769, t2782, t4533, t886, t10513, t15011, t15030, t15038, t2765, t2772, t41060, t41063, t41067, t4487, t4534, t51227, t51231, t51234, t51237, t51240, t51241);
        let (t51256, t51260, t51263, t51264, t51268) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2790::<F>(t10995, t11049, t14990, t14986, t2453, t10506, t2458, t4470, t10069, t14482, t15003, t41020);
        let (t51269, t51272, t51277, t51299, t51306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2791::<F>(t51268, t14939, t213, t4470, t786, t867, t2467, t14567, t2453, t10538, t14662, t251);
        let t51327 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2792::<F>(t213, t225, t40321, t10872, t14502, t14546, t14972, t2646, t39612, t39617, t39622, t4494, t4504, t4514, t50666, t50758, t50916, t51299, t51306, t820, t837, t879);
    (t51184, t51218, t51253, t51256, t51260, t51263, t51264, t51269, t51272, t51277, t51306, t51327)
}
