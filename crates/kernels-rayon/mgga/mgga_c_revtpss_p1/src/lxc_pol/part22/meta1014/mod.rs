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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1014(t11710: f64, t19730: f64, t3091: f64, t11672: f64, t11875: f64, t15604: f64, t15716: f64, t19572: f64, t19645: f64, t19731: f64, t247: f64, t3116: f64, t3117: f64, t42176: f64, t53407: f64, t53413: f64, t53416: f64, t53422: f64, t53427: f64, t53431: f64, t53433: f64, t65071: f64, t19716: f64, t999: f64, t11150: f64, t11703: f64, t13396: f64, t15758: f64, t15936: f64, t15968: f64, t15973: f64, t16095: f64, t1651: f64, t19620: f64, t19726: f64, t19829: f64, t20094: f64, t20099: f64, t20101: f64, t3092: f64, t42254: f64, t43291: f64, t4892: f64, t4893: f64, t4899: f64, t53437: f64, t53479: f64, t54089: f64, t6096: f64, t6100: f64, t20050: f64, t3188: f64, t20054: f64, t1063: f64, t18946: f64, t3109: f64, t11714: f64, t11991: f64, t20046: f64, t3106: f64, t42257: f64, t42270: f64, t42274: f64, t53542: f64, t53557: f64, t53559: f64, t6323: f64, t6327: f64, t6331: f64, t15618: f64, t15682: f64, t2258: f64, t5819: f64, t1062: f64, t53877: f64, t15827: f64, t19878: f64, t1042: f64, t11656: f64, t15592: f64, t15670: f64, t15719: f64, t15725: f64, t15935: f64, t16183: f64, t19702: f64, t19944: f64, t3105: f64, t357: f64, t42324: f64, t42326: f64, t4788: f64, t4839: f64, t53567: f64, t53612: f64, t54471: f64, t15711: f64, t4834: f64, t4181: f64, t11860: f64, t4866: f64, t1066: f64, t11859: f64, t12004: f64, t16089: f64, t19968: f64, t3177: f64, t3184: f64, t42346: f64, t53626: f64, t53628: f64, t63449: f64, t19785: f64, t1045: f64, t4772: f64, t11250: f64, t11632: f64, t11927: f64, t19634: f64, t19636: f64, t19782: f64, t19836: f64, t20089: f64, t3151: f64, t42621: f64, t42643: f64, t43105: f64, t4801: f64, t4905: f64, t53633: f64, t53641: f64, t53643: f64, t54950: f64, t60838: f64, t6271: f64, t15707: f64, t15769: f64, t2251: f64, t12013: f64, t20029: f64, t19671: f64, t3172: f64, t16186: f64, t16199: f64, t16208: f64, t19663: f64, t19672: f64, t3127: f64, t4879: f64, t53473: f64, t54537: f64, t60834: f64, t65365: f64, t65370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t65753 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3491(t11710, t19730, t3091, t11672, t11875, t15604, t15716, t19572, t19645, t19731, t247, t3116, t3117, t42176, t53407, t53413, t53416, t53422, t53427, t53431, t53433, t65071);
        let (t65773, t65795) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3492(t19716, t999, t11150, t11703, t11875, t13396, t15758, t15936, t15968, t15973, t16095, t1651, t19620, t19726, t19829, t20094, t20099, t20101, t3092, t3117, t42254, t43291, t4892, t4893, t4899, t53437, t53479, t54089, t6096, t6100);
        let t65819 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3493(t20050, t3188, t20054, t1063, t18946, t247, t3109, t11714, t11991, t20046, t3106, t42257, t42270, t42274, t53542, t53557, t53559, t6323, t6327, t6331);
        let (t65823, t65829) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3494(t15618, t15682, t2258, t5819);
        let t65852 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3495(t1062, t53877, t15827, t19878, t1042, t1063, t11656, t15592, t15618, t15670, t15719, t15725, t15935, t16183, t19702, t19944, t3105, t3117, t357, t42324, t42326, t4788, t4839, t4893, t4899, t53567, t53612, t54471, t65823, t65829);
        let (t65876, t65881, t65888) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3496(t15711, t4834, t4181, t999, t11860, t4866, t1063, t1066, t11859, t11991, t12004, t16089, t19968, t20046, t20094, t247, t3092, t3117, t3177, t3184, t3188, t42346, t4893, t53626, t53628, t6323, t6327, t63449);
        let t65929 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3497(t11672, t19785, t1045, t4772, t1042, t1063, t11250, t11632, t11859, t11927, t19634, t19636, t19782, t19836, t20089, t3117, t3151, t42621, t42643, t43105, t4801, t4905, t53633, t53641, t53643, t54950, t60838, t6271);
        let (t65931, t65947) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3498(t15707, t15769, t2251, t5819);
        let t65973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3499(t12013, t20029, t1063, t19671, t3172, t1042, t16186, t16199, t16208, t19663, t19672, t3127, t3188, t4801, t4879, t53473, t54537, t60834, t65365, t65370, t65829, t65931, t65947);
    (t65753, t65773, t65795, t65819, t65829, t65852, t65876, t65881, t65888, t65929, t65947, t65973)
}
