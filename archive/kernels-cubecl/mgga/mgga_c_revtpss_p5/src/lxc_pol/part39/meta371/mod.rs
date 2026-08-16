//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta371 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1304;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1305;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1306;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1307;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1308;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta371<F: Float>(t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F, t12131: F, t3095: F, t4823: F, t3096: F, t1087: F, t11773: F, t4801: F, t4181: F, t4786: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F, t11940: F, t1651: F, t3059: F, t3116: F, t11672: F, t11675: F, t11712: F, t11774: F, t3101: F, t3106: F, t3130: F, t4788: F, t4831: F, t4834: F, t3111: F, t11788: F, t3105: F, t3204: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F, t3211: F, t4845: F, t1053: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t1028: F, t11779: F, t11792: F, t11994: F, t1665: F, t4839: F, t4875: F, t12116: F, t4891: F, t4874: F, t4802: F, t4807: F, t11723: F, t11728: F, t11730: F, t11732: F, t11737: F, t11745: F, t4803: F, t4808: F, t4896: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15684, t15687, t15688, t15689, t15691, t15692) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1304::<F>(t11710, t4787, t3091, t245, t4890, t3088, t3317, t1065, t1668, t372, t12131, t3095);
        let (t15693, t15697, t15700, t15702, t15703, t15707) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1305::<F>(t15691, t15692, t372, t4823, t3096, t1087, t11773, t4801, t4181, t4786, t1062, t4857);
        let (t15717, t15722) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1306::<F>(t11986, t1592, t247, t1063, t1062, t11940, t1651, t3059, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725, t15728, t15732, t15736) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1307::<F>(t3111, t4834, t1062, t11788, t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127);
        let t15755 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1308::<F>(t3211, t4845, t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t1665, t4839, t4875);
        let t15779 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1309::<F>(t12116, t4891, t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t3106, t4803, t4808, t4896);
    (t15687, t15688, t15691, t15700, t15702, t15717, t15722, t15755, t15779)
}
