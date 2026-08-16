//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta915 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2949;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2950;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2951;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2952;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2953;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta915<F: Float>(t11466: F, t11507: F, t11554: F, t15413: F, t1634: F, t19021: F, t19294: F, t19297: F, t23711: F, t23761: F, t23785: F, t2987: F, t3012: F, t4707: F, t4708: F, t52443: F, t6190: F, t6205: F, t78303: F, t78305: F, t78307: F, t78309: F, t78311: F, t78313: F, t78315: F, t972: F, t11385: F, t23467: F, t934: F, t11299: F, t4631: F, t6145: F, t23550: F, t41588: F, t23547: F, t2874: F, t23546: F, t2926: F, t2924: F, t19255: F, t23466: F, t41499: F, t41502: F, t19330: F, t41361: F, t41908: F, t51978: F, t52397: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F, t52406: F, t52407: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t11409: F, t15104: F, t15266: F, t15406: F, t19263: F, t19266: F, t19269: F, t23706: F, t311: F, t52812: F, t77598: F, t953: F, t300: F, t77637: F, t77873: F, t78155: F, t78196: F, t78240: F, t78279: F, t77492: F, t77494: F, t77496: F, t77498: F, t77600: F, t77604: F, t77612: F, t77622: F, t77624: F, t77628: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t78316 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2949::<F>(t11466, t11507, t11554, t15413, t1634, t19021, t19294, t19297, t23711, t23761, t23785, t2987, t3012, t4707, t4708, t52443, t6190, t6205, t78303, t78305, t78307, t78309, t78311, t78313, t78315, t972);
        let (t78319, t78322, t78325, t78328, t78329) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2950::<F>(t11385, t23467, t934, t11299, t4631, t6145, t23550, t41588, t23547, t2874, t23546, t2926);
        let (t78332, t78335, t78339, t78342, t78375) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2951::<F>(t2924, t78329, t934, t11385, t19255, t4631, t23466, t41499, t41502, t19330, t41361, t41908, t51978, t52397, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t78394 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2952::<F>(t52406, t52407, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let t78398 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2953::<F>(t11409, t11507, t15104, t15266, t15406, t19263, t19266, t19269, t23706, t311, t52812, t6205, t77598, t78319, t78322, t78325, t78328, t78332, t78335, t78339, t78342, t78375, t78394, t953, t972);
        let (t78402, t78403) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2954::<F>(t300, t77637, t77873, t78155, t78196, t78240, t78279, t78316, t78398, t77492, t77494, t77496, t77498, t77600, t77604, t77612, t77622, t77624, t77628);
    (t78319, t78322, t78325, t78328, t78332, t78335, t78339, t78342, t78402, t78403)
}
