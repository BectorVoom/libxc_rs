//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta346 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1161;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1162;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1163;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1164;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1165;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1166;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1167;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta346(t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64, t3923: f64, t9994: f64, t5673: f64, t5674: f64, t5697: f64, t9962: f64, t5701: f64, t13778: f64, t13779: f64, t13781: f64, t13786: f64, t13793: f64, t13797: f64, t13798: f64, t3934: f64, t5671: f64, t9735: f64, t4004: f64, t9840: f64, t1868: f64, t3829: f64, t828: f64, t9942: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64, t3936: f64, t5704: f64, t3924: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t124: f64, t1882: f64, t5609: f64, t9794: f64, t9793: f64, t1410: f64, t9739: f64, t9742: f64, t9745: f64, t1353: f64, t5591: f64, t4012: f64, t3889: f64, t221: f64, t5627: f64, t9921: f64, t3978: f64, t13583: f64, t13585: f64, t13593: f64, t13599: f64, t13612: f64, t13615: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t13620: f64, t13622: f64, t13623: f64, t13624: f64, t13629: f64, t13631: f64, t13633: f64, t13634: f64, t13635: f64, t13636: f64, t13637: f64, t9394: f64, t9415: f64, t9421: f64, t9427: f64, t9546: f64, t13640: f64, t13641: f64, t13643: f64, t13644: f64, t13645: f64, t13646: f64, t13647: f64, t13653: f64, t13655: f64, t9514: f64, t9517: f64, t9521: f64, t9555: f64, t9569: f64, t9574: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13801, t13804, t13805, t13807, t13810) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1161(t5651, t808, t9736, t241, t820, t9991, t3923, t9994, t5673, t5674, t5697, t9962);
        let t13814 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1162(t5701, t9962, t13778, t13779, t13781, t13786, t13793, t13797, t13798, t13801, t13804, t13807, t13810, t3934, t5671, t9735);
        let (t13817, t13821, t13826, t13832) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1163(t4004, t5673, t5674, t9840, t1868, t3829, t828, t9942, t5608, t5675, t9934, t2661);
        let (t13834, t13841, t13845, t13847, t13848) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1164(t3936, t4004, t5704, t3924, t2482, t4000, t814, t136, t550, t220, t124, t1882);
        let t13860 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1165(t13847, t13848, t5675, t13845, t3924, t5673, t5674, t5609, t9794, t9793, t13817, t13821, t13826, t13832, t13834, t13841, t1410, t3934, t5671, t9739, t9742, t9745);
        let (t13869, t13874, t13880, t13881) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1166(t1353, t5591, t4012, t828, t1868, t3889, t221, t5627, t9921, t3978, t13583, t13585, t13593, t13599, t13612, t13615, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t13882 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1167(t13620, t13622, t13623, t13624, t13629, t13631, t13633, t13634, t13635, t13636, t13637, t9394, t9415, t9421, t9427, t9546);
        let t13884 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1168(t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13653, t13655, t9514, t9517, t9521, t9555, t9569, t9574, t9577);
    (t13805, t13814, t13847, t13848, t13860, t13869, t13874, t13880, t13881, t13882, t13884)
}
