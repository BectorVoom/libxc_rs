//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta811 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2962;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2963;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2964;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2965;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2966;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2967;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta811<F: Float>(t15711: F, t3188: F, t1011: F, t15145: F, t15987: F, t15149: F, t15154: F, t15993: F, t15130: F, t15135: F, t11821: F, t140: F, t15140: F, t11672: F, t15592: F, t4915: F, t4919: F, t51861: F, t51865: F, t51993: F, t11710: F, t15614: F, t3091: F, t1063: F, t15937: F, t3172: F, t11656: F, t11675: F, t11927: F, t11991: F, t15596: F, t15965: F, t16128: F, t16140: F, t16152: F, t1675: F, t3117: F, t42580: F, t42606: F, t42904: F, t4786: F, t4831: F, t53885: F, t15682: F, t12078: F, t53552: F, t16183: F, t73: F, t42793: F, t4892: F, t4895: F, t15951: F, t3127: F, t16166: F, t16171: F, t11620: F, t15910: F, t16095: F, t16172: F, t3092: F, t3115: F, t3154: F, t42967: F, t4578: F, t4783: F, t4893: F, t4910: F, t53835: F, t15785: F, t999: F, t4899: F, t4901: F, t16097: F, t1042: F, t11169: F, t11859: F, t11883: F, t11994: F, t15586: F, t15611: F, t15725: F, t16154: F, t42576: F, t42765: F, t43066: F, t4823: F, t4920: F, t51847: F, t16127: F, t43131: F, t16088: F, t3046: F, t380: F, t16139: F, t11933: F, t15922: F, t16089: F, t16098: F, t2853: F, t3181: F, t42637: F, t42656: F, t42658: F, t42660: F, t42662: F, t4772: F, t906: F) -> (F, F, F, F, F, F, F, F) {
        let (t53955, t53958, t53961, t53964, t53967, t53970, t53972) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2962::<F>(t15711, t3188, t1011, t15145, t15987, t15149, t15154, t15993, t15130, t15135, t11821, t140);
        let t53987 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2963::<F>(t1011, t15140, t53972, t11672, t15592, t4915, t4919, t51861, t51865, t51993, t53955, t53958, t53961, t53964, t53967, t53970);
        let t54013 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2964::<F>(t11710, t15614, t3091, t1063, t15937, t3172, t11656, t11672, t11675, t11927, t11991, t15596, t15965, t16128, t16140, t16152, t1675, t3117, t42580, t42606, t42904, t4786, t4831, t53885);
        let (t54014, t54023, t54026, t54037, t54039) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2965::<F>(t11672, t15682, t12078, t53552, t16183, t73, t42793, t4892, t4895, t15951, t3127, t3172);
        let t54049 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2966::<F>(t16166, t3127, t3172, t16171, t11620, t11656, t15910, t16095, t16172, t3092, t3115, t3117, t3154, t42967, t4578, t4783, t4892, t4893, t4910, t53835, t54014, t54023, t54026, t54037, t54039);
        let (t54064, t54083) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2967::<F>(t15785, t999, t42793, t4899, t4901, t11710, t16095, t16097, t1011, t1042, t11169, t11859, t11883, t11994, t15586, t15611, t15725, t16154, t16172, t1675, t3117, t3127, t42576, t42765, t43066, t4823, t4893, t4915, t4920, t51847);
        let (t54089, t54110) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2968::<F>(t16095, t16127, t43131, t16088, t3046, t380, t16139, t3127, t3172, t1042, t11933, t15922, t16089, t16098, t16152, t2853, t3092, t3181, t42637, t42656, t42658, t42660, t42662, t4772, t906);
    (t53987, t54013, t54026, t54049, t54064, t54083, t54089, t54110)
}
