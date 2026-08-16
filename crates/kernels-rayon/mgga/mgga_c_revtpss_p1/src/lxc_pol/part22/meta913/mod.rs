//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta913 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3119;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta913(t1063: f64, t11262: f64, t4802: f64, t4807: f64, t11859: f64, t11922: f64, t15894: f64, t11714: f64, t4817: f64, t12004: f64, t3299: f64, t53401: f64, t11774: f64, t16103: f64, t53405: f64, t16170: f64, t372: f64, t12116: f64, t15688: f64, t11773: f64, t15925: f64, t11783: f64, t4845: f64, t15745: f64, t3215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55061, t55064, t55067, t55070, t55072, t55100) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3119(t1063, t11262, t4802, t4807, t11859, t11922, t15894, t11714, t4817, t12004, t3299, t53401);
        let (t55104, t55122, t55137, t55141, t55148, t55150) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3120(t11774, t16103, t53405, t16170, t372, t12116, t15688, t11773, t15925, t11783, t4845, t15745, t3215);
    (t55061, t55064, t55067, t55070, t55072, t55100, t55104, t55122, t55137, t55141, t55148, t55150)
}
