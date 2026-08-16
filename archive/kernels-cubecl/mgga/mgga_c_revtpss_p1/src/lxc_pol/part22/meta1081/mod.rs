//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1081 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3894;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3895;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3896;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3897;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3898;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3899;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1081<F: Float>(t1357: F, t22387: F, t689: F, t3899: F, t6896: F, t1444: F, t2782: F, t4075: F, t556: F, t6918: F, t22453: F, t47530: F, t5599: F, t5775: F, t10171: F, t1424: F, t1445: F, t22390: F, t4076: F, t4131: F, t4132: F, t47570: F, t47574: F, t47580: F, t47591: F, t49497: F, t49504: F, t49508: F, t6919: F, t74794: F, t74797: F, t74802: F, t74807: F, t1426: F, t6889: F, t786: F, t3917: F, t14090: F, t14100: F, t22432: F, t47603: F, t686: F, t72: F, t22427: F, t2435: F, t1358: F, t212: F, t22307: F, t5774: F, t14114: F, t14216: F, t14145: F, t2482: F, t4114: F, t6843: F, t1432: F, t22379: F, t2470: F, t1437: F, t4104: F, t6861: F, t1385: F, t1399: F, t46392: F, t46398: F, t46401: F, t46412: F, t47957: F, t73937: F, t74167: F, t820: F, t136: F, t2457: F, t3964: F, t6888: F, t1882: F, t5767: F, t1892: F, t5658: F, t13805: F, t14193: F, t21981: F, t22253: F, t22321: F, t3924: F, t4004: F, t4118: F, t47961: F, t47963: F, t47967: F, t47971: F, t5745: F, t5755: F, t73942: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t74810, t74813, t74824, t74826) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3894::<F>(t1357, t22387, t689, t3899, t6896, t1444, t2782, t4075, t556, t6918, t22453, t47530);
        let t74831 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3895::<F>(t5599, t5775, t689, t10171, t1424, t1445, t22390, t4076, t4131, t4132, t47570, t47574, t47580, t47591, t49497, t49504, t49508, t6918, t6919, t74794, t74797, t74802, t74807, t74810, t74813, t74824, t74826);
        let (t74836, t74838, t74843, t74849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3896::<F>(t1426, t6889, t786, t3917, t14090, t14100, t22432, t47603, t686, t72, t22427, t2435);
        let (t74853, t74855, t74862, t74866, t74873) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3897::<F>(t1358, t212, t22307, t689, t5774, t14114, t14216, t14145, t2482, t4114, t6843, t1432, t22379, t2470);
        let t74890 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3898::<F>(t1437, t2482, t4104, t6861, t1432, t22307, t686, t72, t1385, t1399, t46392, t46398, t46401, t46412, t47957, t73937, t74167, t74862, t74866, t74873, t820);
        let (t74893, t74901, t74908, t74922) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3899::<F>(t1437, t2482, t6843, t4104, t136, t2457, t3964, t6888, t1882, t5767, t1892, t5658);
        let t74926 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3900::<F>(t13805, t1399, t14193, t21981, t22253, t22321, t3924, t4004, t4114, t4118, t47961, t47963, t47967, t47971, t5745, t5755, t73942, t74893, t74901, t74908, t74922, t820);
    (t74831, t74836, t74838, t74843, t74849, t74853, t74855, t74890, t74922, t74926)
}
