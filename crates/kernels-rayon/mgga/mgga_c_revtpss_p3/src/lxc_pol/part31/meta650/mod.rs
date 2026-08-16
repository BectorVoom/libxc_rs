//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta650 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2144;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2145;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2146;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2147;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2148;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2149;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2150;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta650(t106655: f64, t7150: f64, t1668: f64, t7810: f64, t73: f64, t1043: f64, t106745: f64, t106764: f64, t1089: f64, t1695: f64, t19421: f64, t25611: f64, t25629: f64, t25640: f64, t25651: f64, t25692: f64, t27415: f64, t27606: f64, t27621: f64, t27652: f64, t27687: f64, t29739: f64, t29748: f64, t29812: f64, t29822: f64, t29830: f64, t29875: f64, t29883: f64, t4866: f64, t4976: f64, t6251: f64, t7151: f64, t7153: f64, t7156: f64, t7160: f64, t7174: f64, t7821: f64, t93502: f64, t19658: f64, t7122: f64, t19920: f64, t25522: f64, t27489: f64, t4817: f64, t100002: f64, t100006: f64, t100025: f64, t100114: f64, t1675: f64, t19677: f64, t19895: f64, t20083: f64, t25569: f64, t27536: f64, t4912: f64, t6263: f64, t6331: f64, t93646: f64, t19882: f64, t7132: f64, t27450: f64, t4820: f64, t100024: f64, t100048: f64, t100051: f64, t100078: f64, t19838: f64, t20105: f64, t25577: f64, t25580: f64, t6273: f64, t93543: f64, t93555: f64, t20054: f64, t20050: f64, t100092: f64, t100097: f64, t100117: f64, t20066: f64, t27493: f64, t6323: f64, t6327: f64, t93611: f64, t93618: f64, t93622: f64, t19785: f64, t25517: f64, t100132: f64, t16509: f64, t16584: f64, t19622: f64, t19636: f64, t19726: f64, t19778: f64, t19782: f64, t20079: f64, t27492: f64, t4896: f64, t4902: f64, t6268: f64, t93597: f64, t93658: f64, t93667: f64, t6317: f64, t7131: f64, t100055: f64, t100160: f64, t100166: f64, t100230: f64, t1068: f64, t15670: f64, t19745: f64, t19864: f64, t19986: f64, t20046: f64, t4831: f64, t4839: f64, t4907: f64, t93752: f64, t100008: f64, t100138: f64, t100141: f64, t100186: f64, t19682: f64, t19688: f64, t19693: f64, t19707: f64, t19722: f64, t19750: f64, t19754: f64, t19758: f64, t19792: f64, t93548: f64, t93670: f64, t99985: f64, t19826: f64, t25509: f64, t20029: f64, t25505: f64, t100074: f64, t100255: f64, t1671: f64, t19651: f64, t19663: f64, t19668: f64, t19672: f64, t19930: f64, t19934: f64, t4875: f64, t6312: f64, t93655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106823, t106824, t106834) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2144(t106655, t7150, t1668, t7810, t73, t1043, t106745, t106764, t1089, t1695, t19421, t25611, t25629, t25640, t25651, t25692, t27415, t27606, t27621, t27652, t27687, t29739, t29748, t29812, t29822, t29830, t29875, t29883, t4866, t4976, t6251, t7151, t7153, t7156, t7160, t7174, t7821, t93502);
        let (t106877, t106913) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2145(t19658, t7122, t19920, t25522, t27489, t4817, t100002, t100006, t100025, t100114, t1675, t19677, t19895, t20083, t25569, t27536, t4912, t6263, t6331, t93646);
        let t106929 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2146(t19882, t7132, t27450, t4820, t100024, t100048, t100051, t100078, t19838, t20105, t25577, t25580, t6273, t6331, t93543, t93555);
        let t106943 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2147(t20054, t7132, t20050, t100092, t100097, t100117, t20066, t25577, t27493, t6323, t6327, t93611, t93618, t93622);
        let t106968 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2148(t19785, t25517, t100132, t16509, t16584, t19622, t19636, t19726, t19778, t19782, t20079, t27492, t27493, t4896, t4902, t6268, t93597, t93658, t93667);
        let t106990 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2149(t6317, t7131, t100025, t100055, t100160, t100166, t100230, t1068, t15670, t1675, t19745, t19864, t19986, t20046, t25580, t27489, t4831, t4839, t4907, t7132, t93752);
        let t107012 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2150(t100008, t100138, t100141, t100186, t19682, t19688, t19693, t19707, t19722, t19750, t19754, t19758, t19792, t25522, t6273, t7132, t93548, t93670, t99985);
        let t107035 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2151(t19826, t25509, t20029, t25505, t100074, t100255, t1671, t19651, t19663, t19668, t19672, t19930, t19934, t27536, t4875, t6312, t7132, t93655);
    (t106823, t106824, t106834, t106877, t106913, t106929, t106943, t106968, t106990, t107012, t107035)
}
