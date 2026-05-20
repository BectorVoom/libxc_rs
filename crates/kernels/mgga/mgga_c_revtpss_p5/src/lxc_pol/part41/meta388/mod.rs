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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta388<F: Float>(t15935: F, t19661: F, t1042: F, t19666: F, t4801: F, t1592: F, t16138: F, t19399: F, t247: F, t3116: F, t18942: F, t4915: F, t1011: F, t1063: F, t11656: F, t11994: F, t11999: F, t16057: F, t16062: F, t16064: F, t3127: F, t4837: F, t6263: F, t6312: F, t18937: F, t4919: F, t18913: F, t16012: F, t18904: F, t18926: F, t18930: F, t1062: F, t6317: F, t3154: F, t4866: F, t4893: F, t3117: F, t11922: F, t6272: F, t3115: F, t1668: F, t3181: F, t372: F, t1045: F, t4574: F, t12131: F, t6266: F, t15691: F, t1068: F, t15689: F, t15700: F, t3106: F, t4892: F, t6331: F, t4579: F, t1043: F, t3155: F, t4817: F, t4834: F, t11933: F, t11956: F, t11967: F, t11972: F, t11989: F, t15830: F, t16121: F, t16226: F, t1675: F, t3211: F, t6273: F, t6278: F, t127: F, t371: F, t6337: F, t3205: F, t6276: F, t1025: F, t4845: F, t4858: F, t3172: F, t6307: F, t3150: F, t4820: F, t4879: F, t11947: F, t15745: F, t16134: F, t16160: F, t16190: F, t1665: F, t1671: F, t3188: F, t6327: F, t6339: F, t999: F, t1066: F, t18946: F, t11725: F, t6092: F, t3109: F, t6100: F, t19572: F, t4894: F, t4900: F, t11774: F, t15926: F, t4899: F, t4912: F, t6323: F, t11860: F, t19501: F, t19611: F, t3095: F, t3092: F, t19414: F, t1651: F, t2857: F, t4181: F, t2852: F, t11703: F, t4910: F, t11859: F, t15850: F, t16095: F, t16165: F, t16218: F, t16220: F, t3091: F) -> (F, F, F, F, F, F, F) {
        let (t19930, t19934, t19940, t19944, t19947) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1294::<F>(t15935, t19661, t1042, t19666, t4801, t1592, t16138, t19399, t247, t3116, t18942, t4915);
        let t19950 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1295::<F>(t1011, t1063, t11656, t11994, t11999, t16057, t16062, t16064, t19930, t19934, t19940, t19944, t19947, t3127, t4837, t6263, t6312);
        let (t19951, t19954, t19957, t19960, t19963, t19968, t19971) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1296::<F>(t18937, t4919, t18913, t16012, t18904, t18926, t4915, t18930, t1062, t6317, t3154, t4866);
        let (t19973, t19977, t19982, t19985) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1297::<F>(t19971, t4893, t3117, t11922, t6272, t3115, t1668, t3181, t372, t1045, t4574, t12131, t6266);
        let t19989 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1298::<F>(t15691, t19985, t1011, t1068, t15689, t15700, t19951, t19954, t19957, t19960, t19963, t19968, t19973, t19977, t19982, t3106, t4892, t6331);
        let t20012 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1299::<F>(t1045, t4579, t15691, t1043, t1592, t3155, t4817, t4834, t11933, t11956, t11967, t11972, t11989, t15700, t15830, t16121, t16226, t1675, t3211, t6273, t6278);
        let (t20017, t20021, t20025, t20030, t20034) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1300::<F>(t127, t371, t6337, t3205, t6276, t1025, t4845, t4858, t3172, t6307, t3150, t4820, t4879);
        let t20036 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1301::<F>(t11947, t15745, t16134, t16160, t16190, t1665, t1671, t20017, t20021, t20025, t20030, t20034, t3188, t6327, t6339);
        let (t20040, t20046, t20051, t20054) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1302::<F>(t1592, t999, t1045, t15691, t1066, t18946, t247, t11725, t6092, t1063, t3109, t6100);
        let t20073 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1303::<F>(t1063, t20054, t19572, t4894, t3117, t4900, t11774, t15926, t20040, t20046, t20051, t3106, t3188, t4892, t4899, t4912, t6323, t6327, t6331);
        let (t20075, t20079, t20083, t20089, t20090) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1304::<F>(t11860, t19501, t3117, t19611, t3095, t3092, t19414, t247, t3116, t1651, t4866, t1045);
        let t20108 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1305::<F>(t20090, t3117, t1651, t2857, t4181, t3092, t2852, t11703, t19611, t4910, t11859, t15850, t16095, t16165, t16218, t16220, t1675, t20075, t20079, t20083, t3091, t3115, t4837);
    (t19950, t19989, t20012, t20036, t20073, t20089, t20108)
}
