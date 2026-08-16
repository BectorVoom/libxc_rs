//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta388 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1295;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1296;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1297;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1298;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1299;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1300;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1301;
use chunk8::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1302;
use chunk9::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1303;
use chunk10::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1304;
use chunk11::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta388(t15935: f64, t19661: f64, t1042: f64, t19666: f64, t4801: f64, t1592: f64, t16138: f64, t19399: f64, t247: f64, t3116: f64, t18942: f64, t4915: f64, t1011: f64, t1063: f64, t11656: f64, t11994: f64, t11999: f64, t16057: f64, t16062: f64, t16064: f64, t3127: f64, t4837: f64, t6263: f64, t6312: f64, t18937: f64, t4919: f64, t18913: f64, t16012: f64, t18904: f64, t18926: f64, t18930: f64, t1062: f64, t6317: f64, t3154: f64, t4866: f64, t4893: f64, t3117: f64, t11922: f64, t6272: f64, t3115: f64, t1668: f64, t3181: f64, t372: f64, t1045: f64, t4574: f64, t12131: f64, t6266: f64, t15691: f64, t1068: f64, t15689: f64, t15700: f64, t3106: f64, t4892: f64, t6331: f64, t4579: f64, t1043: f64, t3155: f64, t4817: f64, t4834: f64, t11933: f64, t11956: f64, t11967: f64, t11972: f64, t11989: f64, t15830: f64, t16121: f64, t16226: f64, t1675: f64, t3211: f64, t6273: f64, t6278: f64, t127: f64, t371: f64, t6337: f64, t3205: f64, t6276: f64, t1025: f64, t4845: f64, t4858: f64, t3172: f64, t6307: f64, t3150: f64, t4820: f64, t4879: f64, t11947: f64, t15745: f64, t16134: f64, t16160: f64, t16190: f64, t1665: f64, t1671: f64, t3188: f64, t6327: f64, t6339: f64, t999: f64, t1066: f64, t18946: f64, t11725: f64, t6092: f64, t3109: f64, t6100: f64, t19572: f64, t4894: f64, t4900: f64, t11774: f64, t15926: f64, t4899: f64, t4912: f64, t6323: f64, t11860: f64, t19501: f64, t19611: f64, t3095: f64, t3092: f64, t19414: f64, t1651: f64, t2857: f64, t4181: f64, t2852: f64, t11703: f64, t4910: f64, t11859: f64, t15850: f64, t16095: f64, t16165: f64, t16218: f64, t16220: f64, t3091: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19930, t19934, t19940, t19944, t19947) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1294(t15935, t19661, t1042, t19666, t4801, t1592, t16138, t19399, t247, t3116, t18942, t4915);
        let t19950 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1295(t1011, t1063, t11656, t11994, t11999, t16057, t16062, t16064, t19930, t19934, t19940, t19944, t19947, t3127, t4837, t6263, t6312);
        let (t19951, t19954, t19957, t19960, t19963, t19968, t19971) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1296(t18937, t4919, t18913, t16012, t18904, t18926, t4915, t18930, t1062, t6317, t3154, t4866);
        let (t19973, t19977, t19982, t19985) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1297(t19971, t4893, t3117, t11922, t6272, t3115, t1668, t3181, t372, t1045, t4574, t12131, t6266);
        let t19989 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1298(t15691, t19985, t1011, t1068, t15689, t15700, t19951, t19954, t19957, t19960, t19963, t19968, t19973, t19977, t19982, t3106, t4892, t6331);
        let t20012 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1299(t1045, t4579, t15691, t1043, t1592, t3155, t4817, t4834, t11933, t11956, t11967, t11972, t11989, t15700, t15830, t16121, t16226, t1675, t3211, t6273, t6278);
        let (t20017, t20021, t20025, t20030, t20034) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1300(t127, t371, t6337, t3205, t6276, t1025, t4845, t4858, t3172, t6307, t3150, t4820, t4879);
        let t20036 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1301(t11947, t15745, t16134, t16160, t16190, t1665, t1671, t20017, t20021, t20025, t20030, t20034, t3188, t6327, t6339);
        let (t20040, t20046, t20051, t20054) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1302(t1592, t999, t1045, t15691, t1066, t18946, t247, t11725, t6092, t1063, t3109, t6100);
        let t20073 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1303(t1063, t20054, t19572, t4894, t3117, t4900, t11774, t15926, t20040, t20046, t20051, t3106, t3188, t4892, t4899, t4912, t6323, t6327, t6331);
        let (t20075, t20079, t20083, t20089, t20090) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1304(t11860, t19501, t3117, t19611, t3095, t3092, t19414, t247, t3116, t1651, t4866, t1045);
        let t20108 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1305(t20090, t3117, t1651, t2857, t4181, t3092, t2852, t11703, t19611, t4910, t11859, t15850, t16095, t16165, t16218, t16220, t1675, t20075, t20079, t20083, t3091, t3115, t4837);
    (t19950, t19989, t20012, t20036, t20073, t20089, t20108)
}
