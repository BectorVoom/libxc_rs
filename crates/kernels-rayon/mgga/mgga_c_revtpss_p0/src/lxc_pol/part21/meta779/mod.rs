//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2774;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2775;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2776;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2777;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2778;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2779;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta779(t14767: f64, t221: f64, t10703: f64, t2674: f64, t2661: f64, t2662: f64, t2754: f64, t4352: f64, t14728: f64, t9775: f64, t1549: f64, t40861: f64, t14779: f64, t40721: f64, t40724: f64, t10777: f64, t14787: f64, t14495: f64, t40834: f64, t826: f64, t241: f64, t820: f64, t849: f64, t14900: f64, t14923: f64, t10811: f64, t14914: f64, t14788: f64, t10886: f64, t14652: f64, t808: f64, t10489: f64, t10770: f64, t10818: f64, t14676: f64, t14791: f64, t14917: f64, t1544: f64, t2430: f64, t2477: f64, t2745: f64, t2749: f64, t40673: f64, t4343: f64, t4364: f64, t4450: f64, t50459: f64, t50560: f64, t50916: f64, t825: f64, t827: f64, t828: f64, t837: f64, t851: f64, t14746: f64, t2703: f64, t14927: f64, t14697: f64, t40672: f64, t10786: f64, t10861: f64, t14494: f64, t14586: f64, t14772: f64, t14785: f64, t14872: f64, t2394: f64, t2724: f64, t2747: f64, t36833: f64, t40782: f64, t40784: f64, t40789: f64, t40792: f64, t40801: f64, t40804: f64, t40810: f64, t40816: f64, t4362: f64, t4366: f64, t50511: f64, t836: f64, t10905: f64, t14825: f64, t14829: f64, t14819: f64, t40517: f64, t14910: f64, t4423: f64, t14741: f64, t2710: f64, t2713: f64, t10744: f64, t14861: f64, t40791: f64, t4442: f64, t14468: f64, t236: f64, t807: f64, t854: f64, t14745: f64, t1548: f64, t2730: f64, t40655: f64, t40822: f64, t40824: f64, t40836: f64, t40838: f64, t40840: f64, t4457: f64, t775: f64, t800: f64, t14742: f64, t2689: f64, t243: f64, t9794: f64, t10760: f64, t14587: f64, t40799: f64, t4372: f64, t9789: f64, t40627: f64, t50451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50933, t50937, t50939, t50941) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2774(t14767, t221, t10703, t2674, t2661, t2662, t2754, t4352, t14728, t9775, t1549, t40861);
        let (t50943, t50947, t50955, t50957) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2775(t14779, t40721, t221, t40724, t10777, t14787, t14495, t40834, t826, t241, t820, t849);
        let t50979 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2776(t14900, t14923, t10811, t14914, t14788, t10886, t14652, t808, t10489, t10770, t10818, t14676, t14791, t14917, t1544, t2430, t2477, t2745, t2749, t40673, t4343, t4364, t4450, t50459, t50560, t50916, t50933, t50937, t50939, t50941, t50943, t50947, t50955, t50957, t825, t827, t828, t837, t851);
        let t51025 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2777(t14746, t2703, t14923, t14927, t10811, t14697, t40672, t828, t10786, t10861, t14494, t14586, t14676, t14772, t14785, t14791, t14872, t2394, t2724, t2745, t2747, t36833, t40782, t40784, t40789, t40792, t40801, t40804, t40810, t40816, t4362, t4364, t4366, t4450, t50511, t50560, t836, t837);
        let (t51026, t51028, t51042, t51047, t51049, t51055) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2778(t10905, t14825, t14829, t14819, t40517, t10811, t14910, t4423, t836, t14741, t2710, t2713);
        let t51072 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2779(t10744, t14861, t808, t40791, t4442, t14468, t236, t807, t854, t10489, t14586, t14745, t14791, t1548, t2430, t2730, t2745, t2749, t40655, t40822, t40824, t40836, t40838, t40840, t4362, t4457, t51026, t51028, t51042, t51047, t51049, t51055, t775, t800);
        let (t51074, t51079, t51081, t51083, t51086) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2780(t14742, t2689, t243, t9794, t10760, t14495, t14587, t40799, t4372, t9789, t40627, t50451);
    (t50979, t51025, t51049, t51072, t51074, t51079, t51081, t51083, t51086)
}
