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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta377<F: Float>(t1655: F, t697: F, t1011: F, t372: F, t4806: F, t15702: F, t15688: F, t3299: F, t1043: F, t905: F, t606: F, t3155: F, t15691: F, t1047: F, t1063: F, t11656: F, t11977: F, t15700: F, t16190: F, t16196: F, t16201: F, t16205: F, t16210: F, t16218: F, t1671: F, t3169: F, t4825: F, t4869: F, t15625: F, t15676: F, t15722: F, t15755: F, t15779: F, t15814: F, t15855: F, t15913: F, t15949: F, t15991: F, t16034: F, t16073: F, t16114: F, t16136: F, t16189: F, t225: F, t385: F, t1096: F, t4772: F, t1079: F, t1651: F, t3269: F, t3270: F, t5015: F, t1073: F, t1076: F, t11190: F, t11224: F, t15579: F, t15886: F, t1647: F, t1652: F, t3047: F, t3052: F, t3063: F, t3261: F, t342: F, t386: F, t4743: F, t4758: F, t4764: F, t4932: F, t4941: F, t4947: F, t989: F, t995: F, t15717: F, t996: F, t1678: F, t3057: F, t15648: F, t16152: F, t15837: F, t4930: F, t994: F, t3046: F, t1000: F, t11187: F, t11201: F, t11220: F, t1680: F, t1696: F, t3043: F, t3058: F, t3060: F, t3264: F, t3271: F, t4752: F, t4773: F, t379: F, t1078: F, t3066: F, t1695: F, t3325: F, t11121: F, t999: F, t1071: F, t3059: F, t1097: F, t11195: F, t3067: F, t3326: F, t4778: F, t4935: F, t5016: F, t378: F, t15654: F, t1086: F, t1089: F, t15920: F, t16076: F, t12073: F, t1082: F, t3075: F, t4975: F, t4781: F, t3298: F, t4866: F, t1024: F, t1087: F, t1090: F, t12097: F, t12154: F, t1689: F, t3204: F, t3223: F, t3278: F, t3287: F, t3292: F, t3295: F, t3322: F, t4857: F, t4964: F, t4970: F, t4984: F, t4992: F, t5012: F) -> (F, F, F, F, F, F, F, F) {
        let (t16220, t16223, t16226, t16229) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1339::<F>(t1655, t697, t1011, t372, t4806, t15702, t15688, t3299, t1043, t905, t606, t3155);
        let t16233 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1340::<F>(t15691, t16229, t1047, t1063, t11656, t11977, t15700, t16190, t16196, t16201, t16205, t16210, t16218, t16220, t16223, t16226, t1671, t3169, t4825, t4869);
        let t16237 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1341::<F>(t15625, t15676, t15722, t15755, t15779, t15814, t15855, t15913, t15949, t15991, t16034, t16073, t16114, t16136, t16189, t16233);
        let t16272 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1342::<F>(t16237, t225, t385, t1096, t4772, t1079, t1651, t3269, t3270, t5015, t1073, t1076, t11190, t11224, t15579, t15886, t1647, t1652, t3047, t3052, t3063, t3261, t342, t386, t4743, t4758, t4764, t4932, t4941, t4947, t989, t995);
        let t16310 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1343::<F>(t15717, t996, t1678, t3057, t15648, t16152, t15837, t4930, t994, t3046, t1000, t11187, t11201, t11220, t1680, t1696, t3043, t3047, t3058, t3060, t3063, t3264, t3271, t4752, t4758, t4764, t4773, t4941, t4947, t995);
        let (t16312, t16314, t16318, t16322, t16327) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1344::<F>(t3057, t379, t1078, t1651, t3066, t1695, t3325, t3269, t3270, t11121, t5015, t999);
        let t16355 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1345::<F>(t1079, t16327, t342, t4930, t1071, t1647, t1695, t3059, t1651, t3325, t1076, t1097, t11195, t16312, t16314, t16318, t16322, t1696, t3052, t3058, t3067, t3271, t3326, t4752, t4778, t4935, t5016, t995);
        let (t16362, t16371, t16374, t16381, t16390, t16393, t16396, t16399) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1346::<F>(t378, t4743, t1678, t989, t15654, t1086, t1089, t15920, t16076, t12073, t1651, t1082, t16152);
        let t16423 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1347::<F>(t1082, t15837, t3075, t4975, t4781, t1071, t3298, t342, t1089, t4866, t1024, t1087, t1090, t12097, t12154, t16381, t16390, t16393, t16396, t16399, t1647, t1689, t3204, t3223, t3278, t3287, t3292, t3295, t3322, t4857, t4964, t4970, t4984, t4992, t5012, t989);
    (t16237, t16272, t16310, t16355, t16362, t16371, t16374, t16423)
}
