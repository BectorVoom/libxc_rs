//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1081 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3894;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3895;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3896;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3897;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3898;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3899;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1081(t1357: f64, t22387: f64, t689: f64, t3899: f64, t6896: f64, t1444: f64, t2782: f64, t4075: f64, t556: f64, t6918: f64, t22453: f64, t47530: f64, t5599: f64, t5775: f64, t10171: f64, t1424: f64, t1445: f64, t22390: f64, t4076: f64, t4131: f64, t4132: f64, t47570: f64, t47574: f64, t47580: f64, t47591: f64, t49497: f64, t49504: f64, t49508: f64, t6919: f64, t74794: f64, t74797: f64, t74802: f64, t74807: f64, t1426: f64, t6889: f64, t786: f64, t3917: f64, t14090: f64, t14100: f64, t22432: f64, t47603: f64, t686: f64, t72: f64, t22427: f64, t2435: f64, t1358: f64, t212: f64, t22307: f64, t5774: f64, t14114: f64, t14216: f64, t14145: f64, t2482: f64, t4114: f64, t6843: f64, t1432: f64, t22379: f64, t2470: f64, t1437: f64, t4104: f64, t6861: f64, t1385: f64, t1399: f64, t46392: f64, t46398: f64, t46401: f64, t46412: f64, t47957: f64, t73937: f64, t74167: f64, t820: f64, t136: f64, t2457: f64, t3964: f64, t6888: f64, t1882: f64, t5767: f64, t1892: f64, t5658: f64, t13805: f64, t14193: f64, t21981: f64, t22253: f64, t22321: f64, t3924: f64, t4004: f64, t4118: f64, t47961: f64, t47963: f64, t47967: f64, t47971: f64, t5745: f64, t5755: f64, t73942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74810, t74813, t74824, t74826) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3894(t1357, t22387, t689, t3899, t6896, t1444, t2782, t4075, t556, t6918, t22453, t47530);
        let t74831 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3895(t5599, t5775, t689, t10171, t1424, t1445, t22390, t4076, t4131, t4132, t47570, t47574, t47580, t47591, t49497, t49504, t49508, t6918, t6919, t74794, t74797, t74802, t74807, t74810, t74813, t74824, t74826);
        let (t74836, t74838, t74843, t74849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3896(t1426, t6889, t786, t3917, t14090, t14100, t22432, t47603, t686, t72, t22427, t2435);
        let (t74853, t74855, t74862, t74866, t74873) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3897(t1358, t212, t22307, t689, t5774, t14114, t14216, t14145, t2482, t4114, t6843, t1432, t22379, t2470);
        let t74890 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3898(t1437, t2482, t4104, t6861, t1432, t22307, t686, t72, t1385, t1399, t46392, t46398, t46401, t46412, t47957, t73937, t74167, t74862, t74866, t74873, t820);
        let (t74893, t74901, t74908, t74922) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3899(t1437, t2482, t6843, t4104, t136, t2457, t3964, t6888, t1882, t5767, t1892, t5658);
        let t74926 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3900(t13805, t1399, t14193, t21981, t22253, t22321, t3924, t4004, t4114, t4118, t47961, t47963, t47967, t47971, t5745, t5755, t73942, t74893, t74901, t74908, t74922, t820);
    (t74831, t74836, t74838, t74843, t74849, t74853, t74855, t74890, t74922, t74926)
}
