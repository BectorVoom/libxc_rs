//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2139;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2140;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2141;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2142;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta649<F: Float>(t11064: F, t1468: F, t27384: F, t605: F, t6079: F, t198: F, t7850: F, t5824: F, t890: F, t6075: F, t27383: F, t18392: F, t30: F, t1583: F, t4343: F, t25207: F, t18280: F, t1940: F, t1963: F, t2403: F, t25206: F, t25445: F, t27169: F, t27368: F, t27376: F, t27382: F, t27385: F, t27387: F, t29599: F, t29705: F, t7010: F, t7087: F, t7091: F, t7783: F, t92819: F, t98637: F, t106497: F, t106543: F, t106588: F, t1032: F, t6343: F, t1982: F, t3303: F, t4894: F, t100681: F, t1089: F, t1096: F, t1668: F, t1695: F, t1976: F, t19855: F, t20219: F, t25464: F, t25605: F, t25699: F, t27426: F, t27427: F, t27543: F, t27609: F, t27656: F, t29731: F, t29743: F, t29760: F, t29817: F, t4758: F, t4772: F, t7102: F, t7144: F, t7145: F, t7159: F, t7160: F, t7162: F, t7818: F, t7821: F, t7833: F, t93497: F, t93498: F, t93904: F, t94085: F, t99709: F, t99886: F, t99915: F, t29807: F, t342: F, t355: F, t99566: F, t19462: F, t4900: F, t1000: F, t100586: F, t1043: F, t1097: F, t19429: F, t25461: F, t25473: F, t27411: F, t27419: F, t27580: F, t29739: F, t29751: F, t29871: F, t29884: F, t29887: F, t29888: F, t4866: F, t6258: F, t6392: F, t7135: F, t7151: F, t7828: F, t93897: F, t94016: F, t94080: F, t4910: F, t6299: F, t73: F, t3153: F, t19403: F, t20195: F, t25658: F, t27595: F, t27640: F, t27661: F, t27664: F, t27669: F, t29759: F, t4983: F, t4998: F, t6245: F, t6351: F, t7140: F, t93436: F, t93438: F, t93502: F, t93890: F, t93968: F, t94023: F, t94063: F, t94122: F, t99953: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t106590, t106593, t106596, t106602, t106606, t106610, t106611, t106618) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2139::<F>(t11064, t1468, t27384, t605, t6079, t198, t7850, t5824, t890, t6075, t27383, t18392, t30);
        let (t106625, t106636) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2140::<F>(t1583, t4343, t25207, t106590, t106593, t106596, t106602, t106606, t106611, t106618, t18280, t1940, t1963, t2403, t25206, t25445, t27169, t27368, t27376, t27382, t27385, t27387, t29599, t29705, t5824, t7010, t7087, t7091, t7783, t92819, t98637);
        let (t106638, t106655, t106659, t106684) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2141::<F>(t106497, t106543, t106588, t106636, t1032, t6343, t1982, t3303, t4894, t100681, t1089, t1096, t1668, t1695, t1976, t19855, t20219, t25464, t25605, t25699, t27426, t27427, t27543, t27609, t27656, t29731, t29743, t29760, t29817, t4758, t4772, t7102, t7144, t7145, t7159, t7160, t7162, t7818, t7821, t7833, t93497, t93498, t93904, t94085, t99709, t99886, t99915);
        let (t106719, t106730, t106738) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2142::<F>(t29807, t342, t355, t99566, t19462, t1976, t3303, t4900, t1000, t100586, t1043, t106659, t1089, t1096, t1097, t1668, t19429, t25461, t25464, t25473, t25605, t25699, t27411, t27419, t27580, t29739, t29743, t29751, t29871, t29884, t29887, t29888, t4866, t6258, t6392, t7135, t7145, t7151, t7159, t7160, t7828, t93497, t93897, t94016, t94080);
        let (t106745, t106764, t106786) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2143::<F>(t355, t4910, t1976, t6299, t73, t3153, t1043, t106719, t106730, t1089, t1096, t19403, t20195, t25605, t25658, t27595, t27640, t27661, t27664, t27669, t29731, t29739, t29743, t29751, t29759, t4983, t4998, t6245, t6351, t7140, t7159, t93436, t93438, t93502, t93890, t93968, t94023, t94063, t94122, t99953);
    (t106596, t106610, t106625, t106638, t106655, t106659, t106684, t106719, t106738, t106745, t106764, t106786)
}
