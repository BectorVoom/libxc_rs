//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta435 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1561;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1562;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1563;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1564;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1565;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1566;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1567;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1568;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1569;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1570;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta435(t19666: f64, t4806: f64, t1042: f64, t16208: f64, t19661: f64, t1065: f64, t6258: f64, t906: f64, t5825: f64, t606: f64, t4801: f64, t1063: f64, t15668: f64, t15675: f64, t15707: f64, t19651: f64, t19659: f64, t19663: f64, t3127: f64, t3169: f64, t4837: f64, t4875: f64, t6302: f64, t5819: f64, t999: f64, t1032: f64, t6235: f64, t1040: f64, t4872: f64, t1651: f64, t905: f64, t4873: f64, t3092: f64, t357: f64, t4866: f64, t4893: f64, t3117: f64, t19450: f64, t4900: f64, t11661: f64, t19501: f64, t1047: f64, t12013: f64, t16067: f64, t16089: f64, t4803: f64, t4808: f64, t4834: f64, t4892: f64, t4899: f64, t6308: f64, t15957: f64, t6266: f64, t16509: f64, t4891: f64, t16584: f64, t1045: f64, t19497: f64, t1043: f64, t11631: f64, t4894: f64, t4910: f64, t11274: f64, t11277: f64, t11789: f64, t11875: f64, t15684: f64, t15906: f64, t16081: f64, t3091: f64, t3115: f64, t4896: f64, t4902: f64, t6312: f64, t6339: f64, t19380: f64, t373: f64, t371: f64, t372: f64, t19463: f64, t366: f64, t3094: f64, t4186: f64, t4781: f64, t4786: f64, t6092: f64, t11703: f64, t11710: f64, t6267: f64, t4583: f64, t4823: f64, t1025: f64, t1028: f64, t15618: f64, t15712: f64, t15724: f64, t3124: f64, t3224: f64, t4788: f64, t6278: f64, t19477: f64, t18909: f64, t4919: f64, t1011: f64, t1041: f64, t11732: f64, t11737: f64, t15656: f64, t15732: f64, t15736: f64, t15744: f64, t15750: f64, t15754: f64, t1665: f64, t4854: f64, t4858: f64, t19456: f64, t247: f64, t3116: f64, t3172: f64, t6311: f64, t3161: f64, t6244: f64, t1668: f64, t4772: f64, t11866: f64, t11927: f64, t15716: f64, t15771: f64, t15774: f64, t15776: f64, t15817: f64, t1671: f64, t4831: f64, t4869: f64, t4879: f64, t6273: f64, t11134: f64, t11890: f64, t15189: f64, t15874: f64, t15875: f64, t15876: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19668, t19672, t19677, t19680) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1560(t19666, t4806, t1042, t16208, t19661, t1065, t6258, t906, t5825, t606);
        let t19685 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1561(t19680, t4801, t1042, t1063, t15668, t15675, t15707, t19651, t19659, t19663, t19668, t19672, t19677, t3127, t3169, t4837, t4875, t6302);
        let (t19688, t19691, t19693, t19697, t19702, t19705) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1562(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040, t5825, t4872, t1651, t905);
        let t19729 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1563(t19705, t4873, t3092, t357, t4866, t4893, t3117, t19450, t4900, t11661, t19501, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
        let (t19731, t19738, t19741, t19745, t19749) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1564(t15957, t6266, t3092, t16509, t4891, t16584, t1045, t19497, t3117, t1043, t11631, t19450);
        let t19763 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1565(t19749, t3117, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t19745, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
        let (t19770, t19773, t19778, t19781) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1566(t19380, t373, t371, t372, t19463, t366, t3094, t4186, t4781, t3092, t4786, t6092);
        let t19797 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1567(t11703, t19781, t11710, t6267, t3091, t4583, t4823, t1042, t1025, t1028, t15618, t15712, t15724, t19770, t19773, t19778, t3124, t3127, t3224, t4788, t6278, t6302);
        let t19813 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1568(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
        let (t19819, t19827, t19829, t19831, t19836) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1569(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
        let t19841 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1570(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1571(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19680, t19685, t19691, t19729, t19763, t19797, t19813, t19829, t19836, t19841, t19855)
}
