//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta974 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3267;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3268;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3269;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3270;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3271;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3272;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3273;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3274;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3275;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta974(t18414: f64, t40799: f64, t9794: f64, t10760: f64, t18418: f64, t18392: f64, t236: f64, t807: f64, t854: f64, t18643: f64, t40731: f64, t10779: f64, t10786: f64, t14931: f64, t61956: f64, t10770: f64, t14547: f64, t14894: f64, t18444: f64, t18469: f64, t2724: f64, t4362: f64, t4364: f64, t50943: f64, t50947: f64, t50954: f64, t50966: f64, t10811: f64, t18647: f64, t18511: f64, t40864: f64, t10905: f64, t18515: f64, t10744: f64, t18409: f64, t808: f64, t40521: f64, t10900: f64, t14468: f64, t1548: f64, t18393: f64, t2430: f64, t2730: f64, t50968: f64, t50974: f64, t5984: f64, t5988: f64, t775: f64, t800: f64, t4423: f64, t40791: f64, t5989: f64, t10890: f64, t5985: f64, t124: f64, t14586: f64, t14791: f64, t221: f64, t36833: f64, t40782: f64, t40784: f64, t40792: f64, t4343: f64, t4433: f64, t50446: f64, t50977: f64, t50982: f64, t51049: f64, t61234: f64, t14686: f64, t18525: f64, t50570: f64, t14923: f64, t18428: f64, t40627: f64, t61837: f64, t18527: f64, t50295: f64, t2745: f64, t2754: f64, t40801: f64, t40804: f64, t40810: f64, t51000: f64, t51006: f64, t51026: f64, t51028: f64, t18353: f64, t2689: f64, t18394: f64, t2703: f64, t10777: f64, t61715: f64, t837: f64, t14872: f64, t18426: f64, t2646: f64, t2747: f64, t2749: f64, t40284: f64, t40836: f64, t51042: f64, t51047: f64, t61791: f64, t18639: f64, t18507: f64, t18651: f64, t14787: f64, t2723: f64, t40838: f64, t4514: f64, t50459: f64, t51055: f64, t51058: f64, t51060: f64, t6035: f64, t62000: f64, t62002: f64, t18456: f64, t40850: f64, t40851: f64, t51070: f64, t51074: f64, t51078: f64, t51081: f64, t51083: f64, t51086: f64, t51089: f64, t51092: f64, t14662: f64, t14671: f64, t18632: f64, t14494: f64, t1559: f64, t18637: f64, t2477: f64, t4365: f64, t51095: f64, t51098: f64, t51100: f64, t51102: f64, t51104: f64, t51106: f64, t828: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62012, t62015, t62021, t62029, t62033) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3267(t18414, t40799, t9794, t10760, t18418, t18392, t236, t807, t854, t18643, t40731, t10779, t10786, t14931, t61956);
        let t62039 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3268(t10770, t14547, t14894, t18444, t18469, t2724, t4362, t4364, t50943, t50947, t50954, t50966, t62012, t62015, t62021, t62029, t62033);
        let t62074 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3269(t10811, t18647, t18511, t40864, t10905, t18515, t10744, t18409, t808, t18414, t40521, t10900, t14468, t1548, t18393, t18444, t2430, t2724, t2730, t4362, t4364, t50968, t50974, t5984, t5988, t775, t800);
        let (t62080, t62101) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3270(t4423, t775, t40791, t5989, t10890, t5985, t124, t14586, t14791, t221, t2730, t36833, t40782, t40784, t40792, t4343, t4362, t4433, t50446, t50977, t50982, t51049, t61234, t800);
        let t62123 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3271(t14686, t18525, t50570, t61956, t14923, t18428, t10760, t40627, t61837, t18527, t50295, t18444, t2745, t2754, t40801, t40804, t40810, t4364, t51000, t51006, t51026, t51028);
        let t62158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3272(t18353, t2689, t18394, t2703, t10777, t14686, t61715, t837, t14872, t14894, t18426, t18444, t2646, t2745, t2747, t2749, t40284, t40836, t4364, t51042, t51047, t61791);
        let t62186 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3273(t10811, t18639, t10905, t18507, t10777, t10779, t2749, t61715, t18651, t14787, t18426, t2430, t2723, t2745, t2747, t40838, t4362, t4514, t50459, t51055, t51058, t51060, t6035, t62000, t62002);
        let t62199 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3274(t14923, t18456, t40850, t40851, t51070, t51074, t51078, t51081, t51083, t51086, t51089, t51092);
        let (t62209, t62231) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3275(t14662, t2723, t14671, t14686, t14931, t18632, t14494, t14791, t1559, t18426, t18637, t2477, t2745, t2754, t4362, t4364, t4365, t51095, t51098, t51100, t51102, t51104, t51106, t61234, t62080, t828, t851);
    (t62039, t62074, t62101, t62123, t62158, t62186, t62199, t62209, t62231)
}
