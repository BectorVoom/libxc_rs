//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta375 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1330;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1331;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1332;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1333;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1334;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1335;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta375<F: Float>(t16048: F, t3299: F, t11922: F, t4895: F, t4892: F, t140: F, t4886: F, t1011: F, t3241: F, t4924: F, t12047: F, t15905: F, t3151: F, t357: F, t15907: F, t3117: F, t11883: F, t11888: F, t16037: F, t16040: F, t16045: F, t16049: F, t1656: F, t3115: F, t4887: F, t4896: F, t4902: F, t1651: F, t3133: F, t1045: F, t12167: F, t11631: F, t3057: F, t380: F, t3088: F, t370: F, t4757: F, t906: F, t3092: F, t994: F, t606: F, t999: F, t4578: F, t905: F, t15691: F, t11774: F, t11917: F, t11924: F, t11938: F, t11952: F, t11954: F, t11956: F, t11965: F, t3169: F, t4820: F, t1015: F, t13312: F, t1012: F, t4573: F, t11703: F, t3188: F, t4817: F, t11268: F, t11714: F, t11967: F, t11972: F, t11980: F, t11989: F, t12007: F, t12010: F, t1671: F, t1675: F, t1065: F, t4772: F, t1042: F, t2858: F, t4823: F, t1469: F, t3059: F, t4872: F, t247: F, t3116: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16052, t16057, t16062, t16064, t16067) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1330::<F>(t16048, t3299, t11922, t4895, t4892, t140, t4886, t1011, t3241, t4924, t12047, t15905);
        let t16073 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1331::<F>(t3151, t357, t15907, t3117, t11883, t11888, t16037, t16040, t16045, t16049, t16052, t16057, t16062, t16064, t16067, t1656, t3115, t3241, t4887, t4896, t4902);
        let (t16076, t16078, t16081, t16084, t16087, t16088) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1332::<F>(t1651, t3133, t1045, t3117, t12167, t15905, t11631, t3151, t15907, t3057, t380, t3088, t370);
        let (t16089, t16091, t16095, t16096, t16098, t16103) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1333::<F>(t16087, t16088, t4757, t906, t3092, t380, t994, t606, t999, t4578, t905, t1045);
        let t16114 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1334::<F>(t15691, t16103, t11774, t11917, t11924, t11938, t11952, t11954, t11956, t11965, t16078, t16081, t16084, t16089, t16091, t16095, t16098, t3115);
        let t16136 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1335::<F>(t3169, t4820, t1015, t13312, t1012, t16096, t4573, t11703, t3188, t4817, t1011, t11268, t11714, t11967, t11972, t11980, t11989, t12007, t12010, t16095, t1671, t1675);
        let (t16140, t16144, t16149, t16152, t16154) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1336::<F>(t1065, t4772, t906, t1042, t2858, t4823, t1469, t3059, t4872, t999, t247, t3116);
    (t16073, t16076, t16114, t16136, t16140, t16144, t16149, t16152, t16154)
}
