//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1014 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3491;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3492;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3493;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3494;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3495;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3496;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3497;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3498;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1014<F: Float>(t11710: F, t19730: F, t3091: F, t11672: F, t11875: F, t15604: F, t15716: F, t19572: F, t19645: F, t19731: F, t247: F, t3116: F, t3117: F, t42176: F, t53407: F, t53413: F, t53416: F, t53422: F, t53427: F, t53431: F, t53433: F, t65071: F, t19716: F, t999: F, t11150: F, t11703: F, t13396: F, t15758: F, t15936: F, t15968: F, t15973: F, t16095: F, t1651: F, t19620: F, t19726: F, t19829: F, t20094: F, t20099: F, t20101: F, t3092: F, t42254: F, t43291: F, t4892: F, t4893: F, t4899: F, t53437: F, t53479: F, t54089: F, t6096: F, t6100: F, t20050: F, t3188: F, t20054: F, t1063: F, t18946: F, t3109: F, t11714: F, t11991: F, t20046: F, t3106: F, t42257: F, t42270: F, t42274: F, t53542: F, t53557: F, t53559: F, t6323: F, t6327: F, t6331: F, t15618: F, t15682: F, t2258: F, t5819: F, t1062: F, t53877: F, t15827: F, t19878: F, t1042: F, t11656: F, t15592: F, t15670: F, t15719: F, t15725: F, t15935: F, t16183: F, t19702: F, t19944: F, t3105: F, t357: F, t42324: F, t42326: F, t4788: F, t4839: F, t53567: F, t53612: F, t54471: F, t15711: F, t4834: F, t4181: F, t11860: F, t4866: F, t1066: F, t11859: F, t12004: F, t16089: F, t19968: F, t3177: F, t3184: F, t42346: F, t53626: F, t53628: F, t63449: F, t19785: F, t1045: F, t4772: F, t11250: F, t11632: F, t11927: F, t19634: F, t19636: F, t19782: F, t19836: F, t20089: F, t3151: F, t42621: F, t42643: F, t43105: F, t4801: F, t4905: F, t53633: F, t53641: F, t53643: F, t54950: F, t60838: F, t6271: F, t15707: F, t15769: F, t2251: F, t12013: F, t20029: F, t19671: F, t3172: F, t16186: F, t16199: F, t16208: F, t19663: F, t19672: F, t3127: F, t4879: F, t53473: F, t54537: F, t60834: F, t65365: F, t65370: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t65753 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3491::<F>(t11710, t19730, t3091, t11672, t11875, t15604, t15716, t19572, t19645, t19731, t247, t3116, t3117, t42176, t53407, t53413, t53416, t53422, t53427, t53431, t53433, t65071);
        let (t65773, t65795) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3492::<F>(t19716, t999, t11150, t11703, t11875, t13396, t15758, t15936, t15968, t15973, t16095, t1651, t19620, t19726, t19829, t20094, t20099, t20101, t3092, t3117, t42254, t43291, t4892, t4893, t4899, t53437, t53479, t54089, t6096, t6100);
        let t65819 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3493::<F>(t20050, t3188, t20054, t1063, t18946, t247, t3109, t11714, t11991, t20046, t3106, t42257, t42270, t42274, t53542, t53557, t53559, t6323, t6327, t6331);
        let (t65823, t65829) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3494::<F>(t15618, t15682, t2258, t5819);
        let t65852 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3495::<F>(t1062, t53877, t15827, t19878, t1042, t1063, t11656, t15592, t15618, t15670, t15719, t15725, t15935, t16183, t19702, t19944, t3105, t3117, t357, t42324, t42326, t4788, t4839, t4893, t4899, t53567, t53612, t54471, t65823, t65829);
        let (t65876, t65881, t65888) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3496::<F>(t15711, t4834, t4181, t999, t11860, t4866, t1063, t1066, t11859, t11991, t12004, t16089, t19968, t20046, t20094, t247, t3092, t3117, t3177, t3184, t3188, t42346, t4893, t53626, t53628, t6323, t6327, t63449);
        let t65929 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3497::<F>(t11672, t19785, t1045, t4772, t1042, t1063, t11250, t11632, t11859, t11927, t19634, t19636, t19782, t19836, t20089, t3117, t3151, t42621, t42643, t43105, t4801, t4905, t53633, t53641, t53643, t54950, t60838, t6271);
        let (t65931, t65947) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3498::<F>(t15707, t15769, t2251, t5819);
        let t65973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3499::<F>(t12013, t20029, t1063, t19671, t3172, t1042, t16186, t16199, t16208, t19663, t19672, t3127, t3188, t4801, t4879, t53473, t54537, t60834, t65365, t65370, t65829, t65931, t65947);
    (t65753, t65773, t65795, t65819, t65829, t65852, t65876, t65881, t65888, t65929, t65947, t65973)
}
