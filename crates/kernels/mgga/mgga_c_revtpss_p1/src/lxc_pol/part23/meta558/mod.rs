//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta558 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2118;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2119;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2120;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2121;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2122;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta558<F: Float>(t22393: F, t22418: F, t22430: F, t22459: F, t1343: F, t1353: F, t13599: F, t13600: F, t1450: F, t1868: F, t198: F, t21901: F, t21905: F, t21933: F, t21937: F, t21969: F, t4139: F, t532: F, t5532: F, t5536: F, t5591: F, t5627: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t4147: F, t6781: F, t4140: F, t6836: F, t13615: F, t13620: F, t13623: F, t13634: F, t13635: F, t22187: F, t22189: F, t22192: F, t22194: F, t22196: F, t22197: F, t22198: F, t22199: F, t22200: F, t22201: F, t22202: F, t9394: F, t9415: F, t9593: F, t5537: F, t13643: F, t1448: F, t22205: F, t22206: F, t22207: F, t22208: F, t22209: F, t22211: F, t5541: F, t9421: F, t9427: F, t9429: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F, t9588: F, t6922: F, t566: F, t6816: F, t13664: F, t13682: F, t13683: F, t22214: F, t22215: F, t22216: F, t22217: F, t22218: F, t22219: F, t5542: F, t5778: F, t9524: F, t9542: F, t9854: F, t9865: F, t9868: F, t1312: F, t13426: F, t1518: F, t18220: F, t18227: F, t18245: F, t21814: F, t21881: F, t2322: F, t4248: F, t4292: F, t5523: F, t5920: F, t670: F, t7889: F, t1315: F, t1453: F, t1847: F, t1911: F, t21882: F, t21891: F, t4254: F, t4293: F, t4297: F, t508: F, t511: F, t5528: F, t569: F, t5787: F, t5887: F, t649: F, t651: F, t6765: F, t6773: F, t6934: F, t7732: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22461, t22465) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2118::<F>(t22393, t22418, t22430, t22459, t1343, t1353, t13599, t13600, t1450, t1868, t198, t21901, t21905, t21933, t21937, t21969, t4139, t532, t5532, t5536, t5591, t5627, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let (t22466, t22473) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2119::<F>(t4147, t6781, t4140, t6836, t1353, t13615, t13620, t13623, t13634, t13635, t22187, t22189, t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201, t22202, t4139, t5536, t9394, t9415);
        let (t22475, t22482) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2120::<F>(t6781, t9593, t5537, t5591, t13643, t1448, t22205, t22206, t22207, t22208, t22209, t22211, t5536, t5541, t9421, t9427, t9429, t9514, t9517, t9521, t9546, t9569, t9574, t9577, t9588);
        let (t22483, t22486, t22496, t22504) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2121::<F>(t4147, t6922, t566, t6816, t1448, t1868, t1353, t13664, t13682, t13683, t198, t22214, t22215, t22216, t22217, t22218, t22219, t4139, t4140, t5536, t5541, t5542, t5778, t6836, t9524, t9542, t9854, t9865, t9868);
        let (t22506, t22525) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2122::<F>(t22465, t22473, t22482, t22504, t1312, t13426, t1518, t18220, t18227, t18245, t21814, t21881, t2322, t4248, t4292, t5523, t5920, t670, t7889);
        let t22531 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2123::<F>(t1315, t1453, t1847, t1911, t21814, t21882, t21891, t22506, t22525, t2322, t4248, t4254, t4293, t4297, t508, t511, t5528, t569, t5787, t5887, t649, t651, t6765, t6773, t6934, t7732);
    (t22461, t22466, t22475, t22483, t22486, t22496, t22506, t22525, t22531)
}
