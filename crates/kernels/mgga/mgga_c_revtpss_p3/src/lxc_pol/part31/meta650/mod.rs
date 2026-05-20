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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2144;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2145;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2146;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2147;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2148;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2149;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2150;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta650<F: Float>(t106655: F, t7150: F, t1668: F, t7810: F, t73: F, t1043: F, t106745: F, t106764: F, t1089: F, t1695: F, t19421: F, t25611: F, t25629: F, t25640: F, t25651: F, t25692: F, t27415: F, t27606: F, t27621: F, t27652: F, t27687: F, t29739: F, t29748: F, t29812: F, t29822: F, t29830: F, t29875: F, t29883: F, t4866: F, t4976: F, t6251: F, t7151: F, t7153: F, t7156: F, t7160: F, t7174: F, t7821: F, t93502: F, t19658: F, t7122: F, t19920: F, t25522: F, t27489: F, t4817: F, t100002: F, t100006: F, t100025: F, t100114: F, t1675: F, t19677: F, t19895: F, t20083: F, t25569: F, t27536: F, t4912: F, t6263: F, t6331: F, t93646: F, t19882: F, t7132: F, t27450: F, t4820: F, t100024: F, t100048: F, t100051: F, t100078: F, t19838: F, t20105: F, t25577: F, t25580: F, t6273: F, t93543: F, t93555: F, t20054: F, t20050: F, t100092: F, t100097: F, t100117: F, t20066: F, t27493: F, t6323: F, t6327: F, t93611: F, t93618: F, t93622: F, t19785: F, t25517: F, t100132: F, t16509: F, t16584: F, t19622: F, t19636: F, t19726: F, t19778: F, t19782: F, t20079: F, t27492: F, t4896: F, t4902: F, t6268: F, t93597: F, t93658: F, t93667: F, t6317: F, t7131: F, t100055: F, t100160: F, t100166: F, t100230: F, t1068: F, t15670: F, t19745: F, t19864: F, t19986: F, t20046: F, t4831: F, t4839: F, t4907: F, t93752: F, t100008: F, t100138: F, t100141: F, t100186: F, t19682: F, t19688: F, t19693: F, t19707: F, t19722: F, t19750: F, t19754: F, t19758: F, t19792: F, t93548: F, t93670: F, t99985: F, t19826: F, t25509: F, t20029: F, t25505: F, t100074: F, t100255: F, t1671: F, t19651: F, t19663: F, t19668: F, t19672: F, t19930: F, t19934: F, t4875: F, t6312: F, t93655: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t106823, t106824, t106834) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2144::<F>(t106655, t7150, t1668, t7810, t73, t1043, t106745, t106764, t1089, t1695, t19421, t25611, t25629, t25640, t25651, t25692, t27415, t27606, t27621, t27652, t27687, t29739, t29748, t29812, t29822, t29830, t29875, t29883, t4866, t4976, t6251, t7151, t7153, t7156, t7160, t7174, t7821, t93502);
        let (t106877, t106913) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2145::<F>(t19658, t7122, t19920, t25522, t27489, t4817, t100002, t100006, t100025, t100114, t1675, t19677, t19895, t20083, t25569, t27536, t4912, t6263, t6331, t93646);
        let t106929 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2146::<F>(t19882, t7132, t27450, t4820, t100024, t100048, t100051, t100078, t19838, t20105, t25577, t25580, t6273, t6331, t93543, t93555);
        let t106943 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2147::<F>(t20054, t7132, t20050, t100092, t100097, t100117, t20066, t25577, t27493, t6323, t6327, t93611, t93618, t93622);
        let t106968 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2148::<F>(t19785, t25517, t100132, t16509, t16584, t19622, t19636, t19726, t19778, t19782, t20079, t27492, t27493, t4896, t4902, t6268, t93597, t93658, t93667);
        let t106990 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2149::<F>(t6317, t7131, t100025, t100055, t100160, t100166, t100230, t1068, t15670, t1675, t19745, t19864, t19986, t20046, t25580, t27489, t4831, t4839, t4907, t7132, t93752);
        let t107012 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2150::<F>(t100008, t100138, t100141, t100186, t19682, t19688, t19693, t19707, t19722, t19750, t19754, t19758, t19792, t25522, t6273, t7132, t93548, t93670, t99985);
        let t107035 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2151::<F>(t19826, t25509, t20029, t25505, t100074, t100255, t1671, t19651, t19663, t19668, t19672, t19930, t19934, t27536, t4875, t6312, t7132, t93655);
    (t106823, t106824, t106834, t106877, t106913, t106929, t106943, t106968, t106990, t107012, t107035)
}
