//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1588;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1589;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta420(t11986: f64, t1592: f64, t247: f64, t1063: f64, t1062: f64, t11940: f64, t1651: f64, t3059: f64, t3116: f64, t11672: f64, t11675: f64, t11712: f64, t11774: f64, t15684: f64, t15689: f64, t15693: f64, t15697: f64, t15700: f64, t15703: f64, t15707: f64, t3101: f64, t3106: f64, t3130: f64, t4788: f64, t4831: f64, t4834: f64, t3111: f64, t11788: f64, t3105: f64, t3204: f64, t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64, t3211: f64, t4845: f64, t1053: f64, t4857: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t1028: f64, t11779: f64, t11792: f64, t11994: f64, t1665: f64, t4839: f64, t4875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15711, t15717, t15719, t15722) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1588(t11986, t1592, t247, t1063, t1062, t11940, t1651, t3059, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725, t15728, t15731, t15732, t15734, t15736) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1589(t3111, t4834, t1062, t11788, t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127);
        let (t15749, t15752, t15755) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1590(t3211, t4845, t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t1665, t4839, t4875);
    (t15711, t15717, t15719, t15722, t15731, t15734, t15749, t15752, t15755)
}
