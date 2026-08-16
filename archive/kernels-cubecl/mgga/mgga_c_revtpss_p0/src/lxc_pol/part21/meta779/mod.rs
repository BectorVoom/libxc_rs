//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2774;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2775;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2776;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2777;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2778;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2779;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta779<F: Float>(t14767: F, t221: F, t10703: F, t2674: F, t2661: F, t2662: F, t2754: F, t4352: F, t14728: F, t9775: F, t1549: F, t40861: F, t14779: F, t40721: F, t40724: F, t10777: F, t14787: F, t14495: F, t40834: F, t826: F, t241: F, t820: F, t849: F, t14900: F, t14923: F, t10811: F, t14914: F, t14788: F, t10886: F, t14652: F, t808: F, t10489: F, t10770: F, t10818: F, t14676: F, t14791: F, t14917: F, t1544: F, t2430: F, t2477: F, t2745: F, t2749: F, t40673: F, t4343: F, t4364: F, t4450: F, t50459: F, t50560: F, t50916: F, t825: F, t827: F, t828: F, t837: F, t851: F, t14746: F, t2703: F, t14927: F, t14697: F, t40672: F, t10786: F, t10861: F, t14494: F, t14586: F, t14772: F, t14785: F, t14872: F, t2394: F, t2724: F, t2747: F, t36833: F, t40782: F, t40784: F, t40789: F, t40792: F, t40801: F, t40804: F, t40810: F, t40816: F, t4362: F, t4366: F, t50511: F, t836: F, t10905: F, t14825: F, t14829: F, t14819: F, t40517: F, t14910: F, t4423: F, t14741: F, t2710: F, t2713: F, t10744: F, t14861: F, t40791: F, t4442: F, t14468: F, t236: F, t807: F, t854: F, t14745: F, t1548: F, t2730: F, t40655: F, t40822: F, t40824: F, t40836: F, t40838: F, t40840: F, t4457: F, t775: F, t800: F, t14742: F, t2689: F, t243: F, t9794: F, t10760: F, t14587: F, t40799: F, t4372: F, t9789: F, t40627: F, t50451: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50933, t50937, t50939, t50941) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2774::<F>(t14767, t221, t10703, t2674, t2661, t2662, t2754, t4352, t14728, t9775, t1549, t40861);
        let (t50943, t50947, t50955, t50957) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2775::<F>(t14779, t40721, t221, t40724, t10777, t14787, t14495, t40834, t826, t241, t820, t849);
        let t50979 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2776::<F>(t14900, t14923, t10811, t14914, t14788, t10886, t14652, t808, t10489, t10770, t10818, t14676, t14791, t14917, t1544, t2430, t2477, t2745, t2749, t40673, t4343, t4364, t4450, t50459, t50560, t50916, t50933, t50937, t50939, t50941, t50943, t50947, t50955, t50957, t825, t827, t828, t837, t851);
        let t51025 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2777::<F>(t14746, t2703, t14923, t14927, t10811, t14697, t40672, t828, t10786, t10861, t14494, t14586, t14676, t14772, t14785, t14791, t14872, t2394, t2724, t2745, t2747, t36833, t40782, t40784, t40789, t40792, t40801, t40804, t40810, t40816, t4362, t4364, t4366, t4450, t50511, t50560, t836, t837);
        let (t51026, t51028, t51042, t51047, t51049, t51055) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2778::<F>(t10905, t14825, t14829, t14819, t40517, t10811, t14910, t4423, t836, t14741, t2710, t2713);
        let t51072 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2779::<F>(t10744, t14861, t808, t40791, t4442, t14468, t236, t807, t854, t10489, t14586, t14745, t14791, t1548, t2430, t2730, t2745, t2749, t40655, t40822, t40824, t40836, t40838, t40840, t4362, t4457, t51026, t51028, t51042, t51047, t51049, t51055, t775, t800);
        let (t51074, t51079, t51081, t51083, t51086) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2780::<F>(t14742, t2689, t243, t9794, t10760, t14495, t14587, t40799, t4372, t9789, t40627, t50451);
    (t50979, t51025, t51049, t51072, t51074, t51079, t51081, t51083, t51086)
}
