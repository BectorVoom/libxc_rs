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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2988;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2989;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2990;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2991;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2992;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2993;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2994;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta924(t1063: f64, t23485: f64, t247: f64, t3109: f64, t11922: f64, t23993: f64, t3115: f64, t3181: f64, t372: f64, t6305: f64, t23935: f64, t4899: f64, t11994: f64, t15707: f64, t15830: f64, t16226: f64, t1671: f64, t19693: f64, t19697: f64, t19750: f64, t19878: f64, t20083: f64, t23630: f64, t23635: f64, t23844: f64, t23886: f64, t23929: f64, t23980: f64, t3106: f64, t3117: f64, t3155: f64, t3188: f64, t42621: f64, t42622: f64, t4574: f64, t4869: f64, t4892: f64, t6327: f64, t65613: f64, t65823: f64, t65840: f64, t66565: f64, t66621: f64, t78496: f64, t4772: f64, t11675: f64, t11774: f64, t11875: f64, t11927: f64, t15696: f64, t19730: f64, t19777: f64, t23852: f64, t23908: f64, t3162: f64, t42326: f64, t53613: f64, t6271: f64, t65859: f64, t65892: f64, t65894: f64, t65931: f64, t65960: f64, t65965: f64, t66003: f64, t66017: f64, t66022: f64, t66024: f64, t66029: f64, t15932: f64, t19826: f64, t1065: f64, t23598: f64, t11630: f64, t23829: f64, t3172: f64, t1011: f64, t140: f64, t24016: f64, t1042: f64, t11859: f64, t15926: f64, t19651: f64, t19838: f64, t19895: f64, t19940: f64, t20091: f64, t24013: f64, t24017: f64, t3127: f64, t3241: f64, t42830: f64, t4782: f64, t4825: f64, t53724: f64, t53762: f64, t53807: f64, t53855: f64, t6273: f64, t6308: f64, t65717: f64, t66043: f64, t67052: f64, t906: f64, t41361: f64, t42013: f64, t51978: f64, t52946: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64, t52037: f64, t52955: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t341: f64, t54397: f64, t78900: f64, t15689: f64, t15700: f64, t15745: f64, t19993: f64, t225: f64, t3095: f64, t366: f64, t375: f64, t4893: f64, t53320: f64, t53328: f64, t53728: f64, t53876: f64, t53901: f64, t53955: f64, t6278: f64, t66093: f64, t66139: f64, t66141: f64, t66155: f64, t66158: f64, t66176: f64, t66215: f64, t66218: f64, t66221: f64, t66542: f64, t66777: f64, t77513: f64, t15957: f64, t357: f64, t11710: f64, t23907: f64, t3091: f64, t23912: f64, t1668: f64, t905: f64, t11672: f64, t11703: f64, t19611: f64, t19636: f64, t19726: f64, t19738: f64, t19776: f64, t23913: f64, t23917: f64, t23921: f64, t23964: f64, t3092: f64, t4786: f64, t4788: f64, t53800: f64, t54037: f64, t606: f64, t6092: f64, t6096: f64, t66261: f64, t66288: f64, t66304: f64, t67528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79219, t79233, t79247, t79253) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2988(t1063, t23485, t247, t3109, t11922, t23993, t3115, t3181, t372, t6305, t23935, t4899);
        let t79255 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2989(t11994, t15707, t15830, t16226, t1671, t19693, t19697, t19750, t19878, t20083, t23630, t23635, t23844, t23886, t23929, t23980, t3106, t3117, t3155, t3188, t42621, t42622, t4574, t4869, t4892, t6327, t65613, t65823, t65840, t66565, t66621, t78496, t79219, t79233, t79247, t79253);
        let (t79275, t79287) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2990(t4772, t6305, t11675, t11774, t11875, t11927, t15696, t19730, t19777, t23852, t23908, t3117, t3162, t3188, t42326, t53613, t6271, t65859, t65892, t65894, t65931, t65960, t65965, t66003, t66017, t66022, t66024, t66029);
        let t79331 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2991(t15932, t19826, t1065, t23598, t11630, t23829, t3172, t1011, t140, t24016, t1042, t11774, t11859, t15707, t15926, t19651, t19838, t19878, t19895, t19940, t20091, t24013, t24017, t3117, t3127, t3155, t3241, t42830, t4782, t4825, t53724, t53762, t53807, t53855, t6273, t6308, t65717, t66043, t67052, t79275, t906);
        let t79366 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2992(t41361, t42013, t51978, t52946, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t79386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2993(t52037, t52955, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t79388, t79395, t79407) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2994(t341, t79366, t79386, t54397, t78900, t15689, t15700, t15745, t19993, t225, t3095, t366, t375, t4893, t53320, t53328, t53728, t53876, t53901, t53955, t6278, t66093, t66139, t66141, t66155, t66158, t66176, t66215, t66218, t66221, t66542, t66777, t77513);
        let (t79410, t79456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2995(t15957, t357, t11710, t23907, t3091, t23912, t1668, t905, t11672, t11675, t11703, t11927, t16226, t19611, t19636, t19726, t19738, t19776, t23908, t23913, t23917, t23921, t23964, t3092, t3117, t3155, t4786, t4788, t53800, t54037, t606, t6092, t6096, t66261, t66288, t66304, t66777, t67528);
    (t79247, t79255, t79275, t79287, t79331, t79388, t79395, t79407, t79410, t79456)
}
