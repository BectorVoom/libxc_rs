//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2139;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2140;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2141;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2142;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta649(t11064: f64, t1468: f64, t27384: f64, t605: f64, t6079: f64, t198: f64, t7850: f64, t5824: f64, t890: f64, t6075: f64, t27383: f64, t18392: f64, t30: f64, t1583: f64, t4343: f64, t25207: f64, t18280: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25445: f64, t27169: f64, t27368: f64, t27376: f64, t27382: f64, t27385: f64, t27387: f64, t29599: f64, t29705: f64, t7010: f64, t7087: f64, t7091: f64, t7783: f64, t92819: f64, t98637: f64, t106497: f64, t106543: f64, t106588: f64, t1032: f64, t6343: f64, t1982: f64, t3303: f64, t4894: f64, t100681: f64, t1089: f64, t1096: f64, t1668: f64, t1695: f64, t1976: f64, t19855: f64, t20219: f64, t25464: f64, t25605: f64, t25699: f64, t27426: f64, t27427: f64, t27543: f64, t27609: f64, t27656: f64, t29731: f64, t29743: f64, t29760: f64, t29817: f64, t4758: f64, t4772: f64, t7102: f64, t7144: f64, t7145: f64, t7159: f64, t7160: f64, t7162: f64, t7818: f64, t7821: f64, t7833: f64, t93497: f64, t93498: f64, t93904: f64, t94085: f64, t99709: f64, t99886: f64, t99915: f64, t29807: f64, t342: f64, t355: f64, t99566: f64, t19462: f64, t4900: f64, t1000: f64, t100586: f64, t1043: f64, t1097: f64, t19429: f64, t25461: f64, t25473: f64, t27411: f64, t27419: f64, t27580: f64, t29739: f64, t29751: f64, t29871: f64, t29884: f64, t29887: f64, t29888: f64, t4866: f64, t6258: f64, t6392: f64, t7135: f64, t7151: f64, t7828: f64, t93897: f64, t94016: f64, t94080: f64, t4910: f64, t6299: f64, t73: f64, t3153: f64, t19403: f64, t20195: f64, t25658: f64, t27595: f64, t27640: f64, t27661: f64, t27664: f64, t27669: f64, t29759: f64, t4983: f64, t4998: f64, t6245: f64, t6351: f64, t7140: f64, t93436: f64, t93438: f64, t93502: f64, t93890: f64, t93968: f64, t94023: f64, t94063: f64, t94122: f64, t99953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106590, t106593, t106596, t106602, t106606, t106610, t106611, t106618) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2139(t11064, t1468, t27384, t605, t6079, t198, t7850, t5824, t890, t6075, t27383, t18392, t30);
        let (t106625, t106636) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2140(t1583, t4343, t25207, t106590, t106593, t106596, t106602, t106606, t106611, t106618, t18280, t1940, t1963, t2403, t25206, t25445, t27169, t27368, t27376, t27382, t27385, t27387, t29599, t29705, t5824, t7010, t7087, t7091, t7783, t92819, t98637);
        let (t106638, t106655, t106659, t106684) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2141(t106497, t106543, t106588, t106636, t1032, t6343, t1982, t3303, t4894, t100681, t1089, t1096, t1668, t1695, t1976, t19855, t20219, t25464, t25605, t25699, t27426, t27427, t27543, t27609, t27656, t29731, t29743, t29760, t29817, t4758, t4772, t7102, t7144, t7145, t7159, t7160, t7162, t7818, t7821, t7833, t93497, t93498, t93904, t94085, t99709, t99886, t99915);
        let (t106719, t106730, t106738) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2142(t29807, t342, t355, t99566, t19462, t1976, t3303, t4900, t1000, t100586, t1043, t106659, t1089, t1096, t1097, t1668, t19429, t25461, t25464, t25473, t25605, t25699, t27411, t27419, t27580, t29739, t29743, t29751, t29871, t29884, t29887, t29888, t4866, t6258, t6392, t7135, t7145, t7151, t7159, t7160, t7828, t93497, t93897, t94016, t94080);
        let (t106745, t106764, t106786) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2143(t355, t4910, t1976, t6299, t73, t3153, t1043, t106719, t106730, t1089, t1096, t19403, t20195, t25605, t25658, t27595, t27640, t27661, t27664, t27669, t29731, t29739, t29743, t29751, t29759, t4983, t4998, t6245, t6351, t7140, t7159, t93436, t93438, t93502, t93890, t93968, t94023, t94063, t94122, t99953);
    (t106596, t106610, t106625, t106638, t106655, t106659, t106684, t106719, t106738, t106745, t106764, t106786)
}
