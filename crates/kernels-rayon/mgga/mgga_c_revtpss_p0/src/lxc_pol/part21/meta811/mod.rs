//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta811 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2962;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2963;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2964;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2965;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2966;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2967;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta811(t15711: f64, t3188: f64, t1011: f64, t15145: f64, t15987: f64, t15149: f64, t15154: f64, t15993: f64, t15130: f64, t15135: f64, t11821: f64, t140: f64, t15140: f64, t11672: f64, t15592: f64, t4915: f64, t4919: f64, t51861: f64, t51865: f64, t51993: f64, t11710: f64, t15614: f64, t3091: f64, t1063: f64, t15937: f64, t3172: f64, t11656: f64, t11675: f64, t11927: f64, t11991: f64, t15596: f64, t15965: f64, t16128: f64, t16140: f64, t16152: f64, t1675: f64, t3117: f64, t42580: f64, t42606: f64, t42904: f64, t4786: f64, t4831: f64, t53885: f64, t15682: f64, t12078: f64, t53552: f64, t16183: f64, t73: f64, t42793: f64, t4892: f64, t4895: f64, t15951: f64, t3127: f64, t16166: f64, t16171: f64, t11620: f64, t15910: f64, t16095: f64, t16172: f64, t3092: f64, t3115: f64, t3154: f64, t42967: f64, t4578: f64, t4783: f64, t4893: f64, t4910: f64, t53835: f64, t15785: f64, t999: f64, t4899: f64, t4901: f64, t16097: f64, t1042: f64, t11169: f64, t11859: f64, t11883: f64, t11994: f64, t15586: f64, t15611: f64, t15725: f64, t16154: f64, t42576: f64, t42765: f64, t43066: f64, t4823: f64, t4920: f64, t51847: f64, t16127: f64, t43131: f64, t16088: f64, t3046: f64, t380: f64, t16139: f64, t11933: f64, t15922: f64, t16089: f64, t16098: f64, t2853: f64, t3181: f64, t42637: f64, t42656: f64, t42658: f64, t42660: f64, t42662: f64, t4772: f64, t906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53955, t53958, t53961, t53964, t53967, t53970, t53972) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2962(t15711, t3188, t1011, t15145, t15987, t15149, t15154, t15993, t15130, t15135, t11821, t140);
        let t53987 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2963(t1011, t15140, t53972, t11672, t15592, t4915, t4919, t51861, t51865, t51993, t53955, t53958, t53961, t53964, t53967, t53970);
        let t54013 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2964(t11710, t15614, t3091, t1063, t15937, t3172, t11656, t11672, t11675, t11927, t11991, t15596, t15965, t16128, t16140, t16152, t1675, t3117, t42580, t42606, t42904, t4786, t4831, t53885);
        let (t54014, t54023, t54026, t54037, t54039) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2965(t11672, t15682, t12078, t53552, t16183, t73, t42793, t4892, t4895, t15951, t3127, t3172);
        let t54049 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2966(t16166, t3127, t3172, t16171, t11620, t11656, t15910, t16095, t16172, t3092, t3115, t3117, t3154, t42967, t4578, t4783, t4892, t4893, t4910, t53835, t54014, t54023, t54026, t54037, t54039);
        let (t54064, t54083) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2967(t15785, t999, t42793, t4899, t4901, t11710, t16095, t16097, t1011, t1042, t11169, t11859, t11883, t11994, t15586, t15611, t15725, t16154, t16172, t1675, t3117, t3127, t42576, t42765, t43066, t4823, t4893, t4915, t4920, t51847);
        let (t54089, t54110) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2968(t16095, t16127, t43131, t16088, t3046, t380, t16139, t3127, t3172, t1042, t11933, t15922, t16089, t16098, t16152, t2853, t3092, t3181, t42637, t42656, t42658, t42660, t42662, t4772, t906);
    (t53987, t54013, t54026, t54049, t54064, t54083, t54089, t54110)
}
