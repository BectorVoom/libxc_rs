//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta420 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1584;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1585;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1586;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1587;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1588;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta420(t11710: f64, t4787: f64, t3091: f64, t245: f64, t4890: f64, t3088: f64, t3317: f64, t1065: f64, t1668: f64, t372: f64, t12131: f64, t3095: f64, t4823: f64, t3096: f64, t1087: f64, t11773: f64, t4801: f64, t4181: f64, t4786: f64, t1062: f64, t4857: f64, t11986: f64, t1592: f64, t247: f64, t1063: f64, t11940: f64, t1651: f64, t3059: f64, t3116: f64, t11672: f64, t11675: f64, t11712: f64, t11774: f64, t3101: f64, t3106: f64, t3130: f64, t4788: f64, t4831: f64, t4834: f64, t3111: f64, t11788: f64, t3105: f64, t3204: f64, t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64, t3211: f64, t4845: f64, t1053: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t1028: f64, t11779: f64, t11792: f64, t11994: f64, t1665: f64, t4839: f64, t4875: f64, t12116: f64, t4891: f64, t4874: f64, t4802: f64, t4807: f64, t11723: f64, t11728: f64, t11730: f64, t11732: f64, t11737: f64, t11745: f64, t4803: f64, t4808: f64, t4896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15684, t15687, t15688, t15689, t15691, t15692) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1584(t11710, t4787, t3091, t245, t4890, t3088, t3317, t1065, t1668, t372, t12131, t3095);
        let (t15693, t15697, t15700, t15702, t15703, t15707) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1585(t15691, t15692, t372, t4823, t3096, t1087, t11773, t4801, t4181, t4786, t1062, t4857);
        let (t15717, t15722) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1586(t11986, t1592, t247, t1063, t1062, t11940, t1651, t3059, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725, t15728, t15732, t15736) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1587(t3111, t4834, t1062, t11788, t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127);
        let t15755 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1588(t3211, t4845, t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t1665, t4839, t4875);
        let t15779 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1589(t12116, t4891, t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t3106, t4803, t4808, t4896);
    (t15687, t15688, t15691, t15700, t15702, t15717, t15722, t15755, t15779)
}
