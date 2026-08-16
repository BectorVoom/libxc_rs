//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta419 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1457;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1458;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1459;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1460;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1461;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1462;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1463;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1464;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta419(t16199: f64, t19661: f64, t1042: f64, t1469: f64, t4186: f64, t4806: f64, t16208: f64, t1065: f64, t6258: f64, t906: f64, t5825: f64, t606: f64, t4801: f64, t1063: f64, t15668: f64, t15675: f64, t15707: f64, t19651: f64, t19659: f64, t3127: f64, t3169: f64, t4837: f64, t4875: f64, t6302: f64, t5819: f64, t999: f64, t1032: f64, t6235: f64, t1040: f64, t4872: f64, t1651: f64, t905: f64, t4873: f64, t3092: f64, t357: f64, t4866: f64, t4893: f64, t3117: f64, t19450: f64, t4900: f64, t11661: f64, t19501: f64, t1047: f64, t12013: f64, t16067: f64, t16089: f64, t4803: f64, t4808: f64, t4834: f64, t4892: f64, t4899: f64, t6308: f64, t15957: f64, t6266: f64, t16509: f64, t4891: f64, t16584: f64, t1045: f64, t19497: f64, t1043: f64, t11631: f64, t4894: f64, t4910: f64, t11274: f64, t11277: f64, t11789: f64, t11875: f64, t15684: f64, t15906: f64, t16081: f64, t3091: f64, t3115: f64, t4896: f64, t4902: f64, t6312: f64, t6339: f64, t19380: f64, t373: f64, t371: f64, t372: f64, t19463: f64, t366: f64, t3094: f64, t4781: f64, t4786: f64, t6092: f64, t11703: f64, t11710: f64, t6267: f64, t4583: f64, t4823: f64, t1025: f64, t1028: f64, t15618: f64, t15712: f64, t15724: f64, t3124: f64, t3224: f64, t4788: f64, t6278: f64, t19477: f64, t18909: f64, t4919: f64, t1011: f64, t1041: f64, t11732: f64, t11737: f64, t15656: f64, t15732: f64, t15736: f64, t15744: f64, t15750: f64, t15754: f64, t1665: f64, t4854: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19663, t19666, t19668, t19672, t19677, t19680) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1457(t16199, t19661, t1042, t1469, t4186, t4806, t16208, t1065, t6258, t906, t5825, t606);
        let t19685 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1458(t19680, t4801, t1042, t1063, t15668, t15675, t15707, t19651, t19659, t19663, t19668, t19672, t19677, t3127, t3169, t4837, t4875, t6302);
        let (t19688, t19691, t19693, t19697, t19702, t19705) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1459(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040, t5825, t4872, t1651, t905);
        let t19729 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1460(t19705, t4873, t3092, t357, t4866, t4893, t3117, t19450, t4900, t11661, t19501, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
        let (t19731, t19738, t19741, t19745, t19749) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1461(t15957, t6266, t3092, t16509, t4891, t16584, t1045, t19497, t3117, t1043, t11631, t19450);
        let t19763 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1462(t19749, t3117, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t19745, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
        let (t19770, t19773, t19778, t19781) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1463(t19380, t373, t371, t372, t19463, t366, t3094, t4186, t4781, t3092, t4786, t6092);
        let t19797 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1464(t11703, t19781, t11710, t6267, t3091, t4583, t4823, t1042, t1025, t1028, t15618, t15712, t15724, t19770, t19773, t19778, t3124, t3127, t3224, t4788, t6278, t6302);
        let t19813 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1465(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
    (t19666, t19680, t19685, t19691, t19729, t19763, t19797, t19813)
}
