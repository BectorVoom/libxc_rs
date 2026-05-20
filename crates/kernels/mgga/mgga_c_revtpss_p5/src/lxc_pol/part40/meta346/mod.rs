//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta346 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1164;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1165;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1166;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1167;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1168;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1169;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1170;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta346<F: Float>(t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t3923: F, t9994: F, t5673: F, t5674: F, t5697: F, t9962: F, t5701: F, t13778: F, t13779: F, t13781: F, t13786: F, t13793: F, t13797: F, t13798: F, t3934: F, t5671: F, t9735: F, t4004: F, t9840: F, t1868: F, t3829: F, t828: F, t9942: F, t5608: F, t5675: F, t9934: F, t2661: F, t3936: F, t5704: F, t3924: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t124: F, t1882: F, t5609: F, t9794: F, t9793: F, t1410: F, t9739: F, t9742: F, t9745: F, t1353: F, t5591: F, t4012: F, t3889: F, t221: F, t5627: F, t9921: F, t3978: F, t13583: F, t13585: F, t13593: F, t13599: F, t13612: F, t13615: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t13620: F, t13622: F, t13623: F, t13624: F, t13629: F, t13631: F, t13633: F, t13634: F, t13635: F, t13636: F, t13637: F, t9394: F, t9415: F, t9421: F, t9427: F, t9546: F, t13640: F, t13641: F, t13643: F, t13644: F, t13645: F, t13646: F, t13647: F, t13653: F, t13655: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13801, t13804, t13805, t13807, t13810) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1164::<F>(t5651, t808, t9736, t241, t820, t9991, t3923, t9994, t5673, t5674, t5697, t9962);
        let t13814 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1165::<F>(t5701, t9962, t13778, t13779, t13781, t13786, t13793, t13797, t13798, t13801, t13804, t13807, t13810, t3934, t5671, t9735);
        let (t13817, t13821, t13826, t13832) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1166::<F>(t4004, t5673, t5674, t9840, t1868, t3829, t828, t9942, t5608, t5675, t9934, t2661);
        let (t13834, t13841, t13845, t13847, t13848) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1167::<F>(t3936, t4004, t5704, t3924, t2482, t4000, t814, t136, t550, t220, t124, t1882);
        let t13860 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1168::<F>(t13847, t13848, t5675, t13845, t3924, t5673, t5674, t5609, t9794, t9793, t13817, t13821, t13826, t13832, t13834, t13841, t1410, t3934, t5671, t9739, t9742, t9745);
        let (t13869, t13874, t13880, t13881) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1169::<F>(t1353, t5591, t4012, t828, t1868, t3889, t221, t5627, t9921, t3978, t13583, t13585, t13593, t13599, t13612, t13615, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t13882 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1170::<F>(t13620, t13622, t13623, t13624, t13629, t13631, t13633, t13634, t13635, t13636, t13637, t9394, t9415, t9421, t9427, t9546);
        let t13884 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1171::<F>(t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13653, t13655, t9514, t9517, t9521, t9555, t9569, t9574, t9577);
    (t13805, t13814, t13847, t13848, t13860, t13869, t13874, t13880, t13881, t13882, t13884)
}
