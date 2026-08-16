//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta424 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1607;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1608;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1609;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1610;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1611;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1612;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta424(t16048: f64, t3299: f64, t11922: f64, t4895: f64, t4892: f64, t140: f64, t4886: f64, t1011: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64, t3151: f64, t357: f64, t15907: f64, t3117: f64, t11883: f64, t11888: f64, t16037: f64, t16040: f64, t16045: f64, t16049: f64, t1656: f64, t3115: f64, t4887: f64, t4896: f64, t4902: f64, t1651: f64, t3133: f64, t1045: f64, t12167: f64, t11631: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t4757: f64, t906: f64, t3092: f64, t994: f64, t606: f64, t999: f64, t4578: f64, t905: f64, t15691: f64, t11774: f64, t11917: f64, t11924: f64, t11938: f64, t11952: f64, t11954: f64, t11956: f64, t11965: f64, t3169: f64, t4820: f64, t1015: f64, t13312: f64, t1012: f64, t4573: f64, t11703: f64, t3188: f64, t4817: f64, t11268: f64, t11714: f64, t11967: f64, t11972: f64, t11980: f64, t11989: f64, t12007: f64, t12010: f64, t1671: f64, t1675: f64, t1065: f64, t4772: f64, t1042: f64, t2858: f64, t4823: f64, t1469: f64, t3059: f64, t4872: f64, t247: f64, t3116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16052, t16057, t16062, t16064, t16067) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1607(t16048, t3299, t11922, t4895, t4892, t140, t4886, t1011, t3241, t4924, t12047, t15905);
        let t16073 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1608(t3151, t357, t15907, t3117, t11883, t11888, t16037, t16040, t16045, t16049, t16052, t16057, t16062, t16064, t16067, t1656, t3115, t3241, t4887, t4896, t4902);
        let (t16076, t16078, t16081, t16084, t16087, t16088) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1609(t1651, t3133, t1045, t3117, t12167, t15905, t11631, t3151, t15907, t3057, t380, t3088, t370);
        let (t16089, t16091, t16095, t16096, t16098, t16103) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1610(t16087, t16088, t4757, t906, t3092, t380, t994, t606, t999, t4578, t905, t1045);
        let t16114 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1611(t15691, t16103, t11774, t11917, t11924, t11938, t11952, t11954, t11956, t11965, t16078, t16081, t16084, t16089, t16091, t16095, t16098, t3115);
        let t16136 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1612(t3169, t4820, t1015, t13312, t1012, t16096, t4573, t11703, t3188, t4817, t1011, t11268, t11714, t11967, t11972, t11980, t11989, t12007, t12010, t16095, t1671, t1675);
        let (t16140, t16144, t16149, t16152, t16154) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1613(t1065, t4772, t906, t1042, t2858, t4823, t1469, t3059, t4872, t999, t247, t3116);
    (t16073, t16076, t16114, t16136, t16140, t16144, t16149, t16152, t16154)
}
