//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta924 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2988;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2989;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2990;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2991;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2992;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2993;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2994;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta924<F: Float>(t1063: F, t23485: F, t247: F, t3109: F, t11922: F, t23993: F, t3115: F, t3181: F, t372: F, t6305: F, t23935: F, t4899: F, t11994: F, t15707: F, t15830: F, t16226: F, t1671: F, t19693: F, t19697: F, t19750: F, t19878: F, t20083: F, t23630: F, t23635: F, t23844: F, t23886: F, t23929: F, t23980: F, t3106: F, t3117: F, t3155: F, t3188: F, t42621: F, t42622: F, t4574: F, t4869: F, t4892: F, t6327: F, t65613: F, t65823: F, t65840: F, t66565: F, t66621: F, t78496: F, t4772: F, t11675: F, t11774: F, t11875: F, t11927: F, t15696: F, t19730: F, t19777: F, t23852: F, t23908: F, t3162: F, t42326: F, t53613: F, t6271: F, t65859: F, t65892: F, t65894: F, t65931: F, t65960: F, t65965: F, t66003: F, t66017: F, t66022: F, t66024: F, t66029: F, t15932: F, t19826: F, t1065: F, t23598: F, t11630: F, t23829: F, t3172: F, t1011: F, t140: F, t24016: F, t1042: F, t11859: F, t15926: F, t19651: F, t19838: F, t19895: F, t19940: F, t20091: F, t24013: F, t24017: F, t3127: F, t3241: F, t42830: F, t4782: F, t4825: F, t53724: F, t53762: F, t53807: F, t53855: F, t6273: F, t6308: F, t65717: F, t66043: F, t67052: F, t906: F, t41361: F, t42013: F, t51978: F, t52946: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F, t52037: F, t52955: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t341: F, t54397: F, t78900: F, t15689: F, t15700: F, t15745: F, t19993: F, t225: F, t3095: F, t366: F, t375: F, t4893: F, t53320: F, t53328: F, t53728: F, t53876: F, t53901: F, t53955: F, t6278: F, t66093: F, t66139: F, t66141: F, t66155: F, t66158: F, t66176: F, t66215: F, t66218: F, t66221: F, t66542: F, t66777: F, t77513: F, t15957: F, t357: F, t11710: F, t23907: F, t3091: F, t23912: F, t1668: F, t905: F, t11672: F, t11703: F, t19611: F, t19636: F, t19726: F, t19738: F, t19776: F, t23913: F, t23917: F, t23921: F, t23964: F, t3092: F, t4786: F, t4788: F, t53800: F, t54037: F, t606: F, t6092: F, t6096: F, t66261: F, t66288: F, t66304: F, t67528: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t79219, t79233, t79247, t79253) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2988::<F>(t1063, t23485, t247, t3109, t11922, t23993, t3115, t3181, t372, t6305, t23935, t4899);
        let t79255 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2989::<F>(t11994, t15707, t15830, t16226, t1671, t19693, t19697, t19750, t19878, t20083, t23630, t23635, t23844, t23886, t23929, t23980, t3106, t3117, t3155, t3188, t42621, t42622, t4574, t4869, t4892, t6327, t65613, t65823, t65840, t66565, t66621, t78496, t79219, t79233, t79247, t79253);
        let (t79275, t79287) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2990::<F>(t4772, t6305, t11675, t11774, t11875, t11927, t15696, t19730, t19777, t23852, t23908, t3117, t3162, t3188, t42326, t53613, t6271, t65859, t65892, t65894, t65931, t65960, t65965, t66003, t66017, t66022, t66024, t66029);
        let t79331 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2991::<F>(t15932, t19826, t1065, t23598, t11630, t23829, t3172, t1011, t140, t24016, t1042, t11774, t11859, t15707, t15926, t19651, t19838, t19878, t19895, t19940, t20091, t24013, t24017, t3117, t3127, t3155, t3241, t42830, t4782, t4825, t53724, t53762, t53807, t53855, t6273, t6308, t65717, t66043, t67052, t79275, t906);
        let t79366 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2992::<F>(t41361, t42013, t51978, t52946, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t79386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2993::<F>(t52037, t52955, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t79388, t79395, t79407) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2994::<F>(t341, t79366, t79386, t54397, t78900, t15689, t15700, t15745, t19993, t225, t3095, t366, t375, t4893, t53320, t53328, t53728, t53876, t53901, t53955, t6278, t66093, t66139, t66141, t66155, t66158, t66176, t66215, t66218, t66221, t66542, t66777, t77513);
        let (t79410, t79456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2995::<F>(t15957, t357, t11710, t23907, t3091, t23912, t1668, t905, t11672, t11675, t11703, t11927, t16226, t19611, t19636, t19726, t19738, t19776, t23908, t23913, t23917, t23921, t23964, t3092, t3117, t3155, t4786, t4788, t53800, t54037, t606, t6092, t6096, t66261, t66288, t66304, t66777, t67528);
    (t79247, t79255, t79275, t79287, t79331, t79388, t79395, t79407, t79410, t79456)
}
