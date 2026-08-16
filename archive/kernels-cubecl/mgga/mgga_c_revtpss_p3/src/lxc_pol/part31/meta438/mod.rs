//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1565;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta438<F: Float>(t19855: F, t341: F, t225: F, t366: F, t15696: F, t4782: F, t4787: F, t1058: F, t6318: F, t1053: F, t6317: F, t4786: F, t6096: F, t3092: F, t1062: F, t15670: F, t247: F, t3109: F, t1063: F, t11672: F, t11774: F, t15796: F, t15829: F, t3091: F, t375: F, t4839: F, t6268: F) -> (F, F, F, F, F, F, F) {
        let (t19856, t19857, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1565::<F>(t19855, t341, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
        let (t19873, t19882, t19885) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1566::<F>(t19872, t3092, t1062, t15670, t247, t3109, t6096, t1063, t11672, t11774, t15796, t15829, t19858, t19861, t19864, t19867, t19869, t3091, t375, t4839, t6268);
    (t19856, t19857, t19861, t19864, t19873, t19882, t19885)
}
