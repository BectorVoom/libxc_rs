//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1588;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1589;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta420<F: Float>(t11986: F, t1592: F, t247: F, t1063: F, t1062: F, t11940: F, t1651: F, t3059: F, t3116: F, t11672: F, t11675: F, t11712: F, t11774: F, t15684: F, t15689: F, t15693: F, t15697: F, t15700: F, t15703: F, t15707: F, t3101: F, t3106: F, t3130: F, t4788: F, t4831: F, t4834: F, t3111: F, t11788: F, t3105: F, t3204: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F, t3211: F, t4845: F, t1053: F, t4857: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t1028: F, t11779: F, t11792: F, t11994: F, t1665: F, t4839: F, t4875: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15711, t15717, t15719, t15722) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1588::<F>(t11986, t1592, t247, t1063, t1062, t11940, t1651, t3059, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725, t15728, t15731, t15732, t15734, t15736) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1589::<F>(t3111, t4834, t1062, t11788, t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127);
        let (t15749, t15752, t15755) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1590::<F>(t3211, t4845, t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t1665, t4839, t4875);
    (t15711, t15717, t15719, t15722, t15731, t15734, t15749, t15752, t15755)
}
