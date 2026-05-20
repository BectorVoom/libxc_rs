//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta387 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1280;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1281;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1282;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1283;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1284;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1285;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1286;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1287;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta387<F: Float>(t16199: F, t19661: F, t1042: F, t1469: F, t4186: F, t4806: F, t16208: F, t1065: F, t6258: F, t906: F, t5825: F, t606: F, t4801: F, t1063: F, t15668: F, t15675: F, t15707: F, t19651: F, t19659: F, t3127: F, t3169: F, t4837: F, t4875: F, t6302: F, t5819: F, t999: F, t1032: F, t6235: F, t1040: F, t4872: F, t1651: F, t905: F, t4873: F, t3092: F, t357: F, t4866: F, t4893: F, t3117: F, t19450: F, t4900: F, t11661: F, t19501: F, t1047: F, t12013: F, t16067: F, t16089: F, t4803: F, t4808: F, t4834: F, t4892: F, t4899: F, t6308: F, t15957: F, t6266: F, t16509: F, t4891: F, t16584: F, t1045: F, t19497: F, t1043: F, t11631: F, t4894: F, t4910: F, t11274: F, t11277: F, t11789: F, t11875: F, t15684: F, t15906: F, t16081: F, t3091: F, t3115: F, t4896: F, t4902: F, t6312: F, t6339: F, t19380: F, t373: F, t371: F, t372: F, t19463: F, t366: F, t3094: F, t4781: F, t4786: F, t6092: F, t11703: F, t11710: F, t6267: F, t4583: F, t4823: F, t1025: F, t1028: F, t15618: F, t15712: F, t15724: F, t3124: F, t3224: F, t4788: F, t6278: F, t19477: F, t18909: F, t4919: F, t1011: F, t1041: F, t11732: F, t11737: F, t15656: F, t15732: F, t15736: F, t15744: F, t15750: F, t15754: F, t1665: F, t4854: F, t4858: F) -> (F, F, F, F, F, F, F, F) {
        let (t19663, t19666, t19668, t19672, t19677, t19680) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1280::<F>(t16199, t19661, t1042, t1469, t4186, t4806, t16208, t1065, t6258, t906, t5825, t606);
        let t19685 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1281::<F>(t19680, t4801, t1042, t1063, t15668, t15675, t15707, t19651, t19659, t19663, t19668, t19672, t19677, t3127, t3169, t4837, t4875, t6302);
        let (t19688, t19691, t19693, t19697, t19702, t19705) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1282::<F>(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040, t5825, t4872, t1651, t905);
        let t19729 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1283::<F>(t19705, t4873, t3092, t357, t4866, t4893, t3117, t19450, t4900, t11661, t19501, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
        let (t19731, t19738, t19741, t19745, t19749) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1284::<F>(t15957, t6266, t3092, t16509, t4891, t16584, t1045, t19497, t3117, t1043, t11631, t19450);
        let t19763 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1285::<F>(t19749, t3117, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t19745, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
        let (t19770, t19773, t19778, t19781) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1286::<F>(t19380, t373, t371, t372, t19463, t366, t3094, t4186, t4781, t3092, t4786, t6092);
        let t19797 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1287::<F>(t11703, t19781, t11710, t6267, t3091, t4583, t4823, t1042, t1025, t1028, t15618, t15712, t15724, t19770, t19773, t19778, t3124, t3127, t3224, t4788, t6278, t6302);
        let t19813 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1288::<F>(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
    (t19666, t19680, t19685, t19691, t19729, t19763, t19797, t19813)
}
