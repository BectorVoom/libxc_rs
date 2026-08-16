//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta915 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2949;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2950;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2951;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2952;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2953;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta915(t11466: f64, t11507: f64, t11554: f64, t15413: f64, t1634: f64, t19021: f64, t19294: f64, t19297: f64, t23711: f64, t23761: f64, t23785: f64, t2987: f64, t3012: f64, t4707: f64, t4708: f64, t52443: f64, t6190: f64, t6205: f64, t78303: f64, t78305: f64, t78307: f64, t78309: f64, t78311: f64, t78313: f64, t78315: f64, t972: f64, t11385: f64, t23467: f64, t934: f64, t11299: f64, t4631: f64, t6145: f64, t23550: f64, t41588: f64, t23547: f64, t2874: f64, t23546: f64, t2926: f64, t2924: f64, t19255: f64, t23466: f64, t41499: f64, t41502: f64, t19330: f64, t41361: f64, t41908: f64, t51978: f64, t52397: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64, t52406: f64, t52407: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t11409: f64, t15104: f64, t15266: f64, t15406: f64, t19263: f64, t19266: f64, t19269: f64, t23706: f64, t311: f64, t52812: f64, t77598: f64, t953: f64, t300: f64, t77637: f64, t77873: f64, t78155: f64, t78196: f64, t78240: f64, t78279: f64, t77492: f64, t77494: f64, t77496: f64, t77498: f64, t77600: f64, t77604: f64, t77612: f64, t77622: f64, t77624: f64, t77628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t78316 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2949(t11466, t11507, t11554, t15413, t1634, t19021, t19294, t19297, t23711, t23761, t23785, t2987, t3012, t4707, t4708, t52443, t6190, t6205, t78303, t78305, t78307, t78309, t78311, t78313, t78315, t972);
        let (t78319, t78322, t78325, t78328, t78329) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2950(t11385, t23467, t934, t11299, t4631, t6145, t23550, t41588, t23547, t2874, t23546, t2926);
        let (t78332, t78335, t78339, t78342, t78375) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2951(t2924, t78329, t934, t11385, t19255, t4631, t23466, t41499, t41502, t19330, t41361, t41908, t51978, t52397, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t78394 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2952(t52406, t52407, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let t78398 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2953(t11409, t11507, t15104, t15266, t15406, t19263, t19266, t19269, t23706, t311, t52812, t6205, t77598, t78319, t78322, t78325, t78328, t78332, t78335, t78339, t78342, t78375, t78394, t953, t972);
        let (t78402, t78403) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2954(t300, t77637, t77873, t78155, t78196, t78240, t78279, t78316, t78398, t77492, t77494, t77496, t77498, t77600, t77604, t77612, t77622, t77624, t77628);
    (t78319, t78322, t78325, t78328, t78332, t78335, t78339, t78342, t78402, t78403)
}
