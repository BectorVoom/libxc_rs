//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta377 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1340;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1341;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1342;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1343;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1344;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1345;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1346;
use chunk8::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta377(t1655: f64, t697: f64, t1011: f64, t372: f64, t4806: f64, t15702: f64, t15688: f64, t3299: f64, t1043: f64, t905: f64, t606: f64, t3155: f64, t15691: f64, t1047: f64, t1063: f64, t11656: f64, t11977: f64, t15700: f64, t16190: f64, t16196: f64, t16201: f64, t16205: f64, t16210: f64, t16218: f64, t1671: f64, t3169: f64, t4825: f64, t4869: f64, t15625: f64, t15676: f64, t15722: f64, t15755: f64, t15779: f64, t15814: f64, t15855: f64, t15913: f64, t15949: f64, t15991: f64, t16034: f64, t16073: f64, t16114: f64, t16136: f64, t16189: f64, t225: f64, t385: f64, t1096: f64, t4772: f64, t1079: f64, t1651: f64, t3269: f64, t3270: f64, t5015: f64, t1073: f64, t1076: f64, t11190: f64, t11224: f64, t15579: f64, t15886: f64, t1647: f64, t1652: f64, t3047: f64, t3052: f64, t3063: f64, t3261: f64, t342: f64, t386: f64, t4743: f64, t4758: f64, t4764: f64, t4932: f64, t4941: f64, t4947: f64, t989: f64, t995: f64, t15717: f64, t996: f64, t1678: f64, t3057: f64, t15648: f64, t16152: f64, t15837: f64, t4930: f64, t994: f64, t3046: f64, t1000: f64, t11187: f64, t11201: f64, t11220: f64, t1680: f64, t1696: f64, t3043: f64, t3058: f64, t3060: f64, t3264: f64, t3271: f64, t4752: f64, t4773: f64, t379: f64, t1078: f64, t3066: f64, t1695: f64, t3325: f64, t11121: f64, t999: f64, t1071: f64, t3059: f64, t1097: f64, t11195: f64, t3067: f64, t3326: f64, t4778: f64, t4935: f64, t5016: f64, t378: f64, t15654: f64, t1086: f64, t1089: f64, t15920: f64, t16076: f64, t12073: f64, t1082: f64, t3075: f64, t4975: f64, t4781: f64, t3298: f64, t4866: f64, t1024: f64, t1087: f64, t1090: f64, t12097: f64, t12154: f64, t1689: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t3292: f64, t3295: f64, t3322: f64, t4857: f64, t4964: f64, t4970: f64, t4984: f64, t4992: f64, t5012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16220, t16223, t16226, t16229) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1339(t1655, t697, t1011, t372, t4806, t15702, t15688, t3299, t1043, t905, t606, t3155);
        let t16233 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1340(t15691, t16229, t1047, t1063, t11656, t11977, t15700, t16190, t16196, t16201, t16205, t16210, t16218, t16220, t16223, t16226, t1671, t3169, t4825, t4869);
        let t16237 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1341(t15625, t15676, t15722, t15755, t15779, t15814, t15855, t15913, t15949, t15991, t16034, t16073, t16114, t16136, t16189, t16233);
        let t16272 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1342(t16237, t225, t385, t1096, t4772, t1079, t1651, t3269, t3270, t5015, t1073, t1076, t11190, t11224, t15579, t15886, t1647, t1652, t3047, t3052, t3063, t3261, t342, t386, t4743, t4758, t4764, t4932, t4941, t4947, t989, t995);
        let t16310 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1343(t15717, t996, t1678, t3057, t15648, t16152, t15837, t4930, t994, t3046, t1000, t11187, t11201, t11220, t1680, t1696, t3043, t3047, t3058, t3060, t3063, t3264, t3271, t4752, t4758, t4764, t4773, t4941, t4947, t995);
        let (t16312, t16314, t16318, t16322, t16327) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1344(t3057, t379, t1078, t1651, t3066, t1695, t3325, t3269, t3270, t11121, t5015, t999);
        let t16355 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1345(t1079, t16327, t342, t4930, t1071, t1647, t1695, t3059, t1651, t3325, t1076, t1097, t11195, t16312, t16314, t16318, t16322, t1696, t3052, t3058, t3067, t3271, t3326, t4752, t4778, t4935, t5016, t995);
        let (t16362, t16371, t16374, t16381, t16390, t16393, t16396, t16399) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1346(t378, t4743, t1678, t989, t15654, t1086, t1089, t15920, t16076, t12073, t1651, t1082, t16152);
        let t16423 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1347(t1082, t15837, t3075, t4975, t4781, t1071, t3298, t342, t1089, t4866, t1024, t1087, t1090, t12097, t12154, t16381, t16390, t16393, t16396, t16399, t1647, t1689, t3204, t3223, t3278, t3287, t3292, t3295, t3322, t4857, t4964, t4970, t4984, t4992, t5012, t989);
    (t16237, t16272, t16310, t16355, t16362, t16371, t16374, t16423)
}
