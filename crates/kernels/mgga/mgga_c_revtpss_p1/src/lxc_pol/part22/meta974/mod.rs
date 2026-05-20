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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta974<F: Float>(t18414: F, t40799: F, t9794: F, t10760: F, t18418: F, t18392: F, t236: F, t807: F, t854: F, t18643: F, t40731: F, t10779: F, t10786: F, t14931: F, t61956: F, t10770: F, t14547: F, t14894: F, t18444: F, t18469: F, t2724: F, t4362: F, t4364: F, t50943: F, t50947: F, t50954: F, t50966: F, t10811: F, t18647: F, t18511: F, t40864: F, t10905: F, t18515: F, t10744: F, t18409: F, t808: F, t40521: F, t10900: F, t14468: F, t1548: F, t18393: F, t2430: F, t2730: F, t50968: F, t50974: F, t5984: F, t5988: F, t775: F, t800: F, t4423: F, t40791: F, t5989: F, t10890: F, t5985: F, t124: F, t14586: F, t14791: F, t221: F, t36833: F, t40782: F, t40784: F, t40792: F, t4343: F, t4433: F, t50446: F, t50977: F, t50982: F, t51049: F, t61234: F, t14686: F, t18525: F, t50570: F, t14923: F, t18428: F, t40627: F, t61837: F, t18527: F, t50295: F, t2745: F, t2754: F, t40801: F, t40804: F, t40810: F, t51000: F, t51006: F, t51026: F, t51028: F, t18353: F, t2689: F, t18394: F, t2703: F, t10777: F, t61715: F, t837: F, t14872: F, t18426: F, t2646: F, t2747: F, t2749: F, t40284: F, t40836: F, t51042: F, t51047: F, t61791: F, t18639: F, t18507: F, t18651: F, t14787: F, t2723: F, t40838: F, t4514: F, t50459: F, t51055: F, t51058: F, t51060: F, t6035: F, t62000: F, t62002: F, t18456: F, t40850: F, t40851: F, t51070: F, t51074: F, t51078: F, t51081: F, t51083: F, t51086: F, t51089: F, t51092: F, t14662: F, t14671: F, t18632: F, t14494: F, t1559: F, t18637: F, t2477: F, t4365: F, t51095: F, t51098: F, t51100: F, t51102: F, t51104: F, t51106: F, t828: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t62012, t62015, t62021, t62029, t62033) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3267::<F>(t18414, t40799, t9794, t10760, t18418, t18392, t236, t807, t854, t18643, t40731, t10779, t10786, t14931, t61956);
        let t62039 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3268::<F>(t10770, t14547, t14894, t18444, t18469, t2724, t4362, t4364, t50943, t50947, t50954, t50966, t62012, t62015, t62021, t62029, t62033);
        let t62074 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3269::<F>(t10811, t18647, t18511, t40864, t10905, t18515, t10744, t18409, t808, t18414, t40521, t10900, t14468, t1548, t18393, t18444, t2430, t2724, t2730, t4362, t4364, t50968, t50974, t5984, t5988, t775, t800);
        let (t62080, t62101) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3270::<F>(t4423, t775, t40791, t5989, t10890, t5985, t124, t14586, t14791, t221, t2730, t36833, t40782, t40784, t40792, t4343, t4362, t4433, t50446, t50977, t50982, t51049, t61234, t800);
        let t62123 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3271::<F>(t14686, t18525, t50570, t61956, t14923, t18428, t10760, t40627, t61837, t18527, t50295, t18444, t2745, t2754, t40801, t40804, t40810, t4364, t51000, t51006, t51026, t51028);
        let t62158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3272::<F>(t18353, t2689, t18394, t2703, t10777, t14686, t61715, t837, t14872, t14894, t18426, t18444, t2646, t2745, t2747, t2749, t40284, t40836, t4364, t51042, t51047, t61791);
        let t62186 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3273::<F>(t10811, t18639, t10905, t18507, t10777, t10779, t2749, t61715, t18651, t14787, t18426, t2430, t2723, t2745, t2747, t40838, t4362, t4514, t50459, t51055, t51058, t51060, t6035, t62000, t62002);
        let t62199 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3274::<F>(t14923, t18456, t40850, t40851, t51070, t51074, t51078, t51081, t51083, t51086, t51089, t51092);
        let (t62209, t62231) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3275::<F>(t14662, t2723, t14671, t14686, t14931, t18632, t14494, t14791, t1559, t18426, t18637, t2477, t2745, t2754, t4362, t4364, t4365, t51095, t51098, t51100, t51102, t51104, t51106, t61234, t62080, t828, t851);
    (t62039, t62074, t62101, t62123, t62158, t62186, t62199, t62209, t62231)
}
