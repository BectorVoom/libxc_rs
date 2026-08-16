//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta913 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3119;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta913<F: Float>(t1063: F, t11262: F, t4802: F, t4807: F, t11859: F, t11922: F, t15894: F, t11714: F, t4817: F, t12004: F, t3299: F, t53401: F, t11774: F, t16103: F, t53405: F, t16170: F, t372: F, t12116: F, t15688: F, t11773: F, t15925: F, t11783: F, t4845: F, t15745: F, t3215: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55061, t55064, t55067, t55070, t55072, t55100) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3119::<F>(t1063, t11262, t4802, t4807, t11859, t11922, t15894, t11714, t4817, t12004, t3299, t53401);
        let (t55104, t55122, t55137, t55141, t55148, t55150) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3120::<F>(t11774, t16103, t53405, t16170, t372, t12116, t15688, t11773, t15925, t11783, t4845, t15745, t3215);
    (t55061, t55064, t55067, t55070, t55072, t55100, t55104, t55122, t55137, t55141, t55148, t55150)
}
