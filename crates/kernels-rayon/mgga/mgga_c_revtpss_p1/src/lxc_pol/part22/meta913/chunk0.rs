//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3119/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3119(t1063: f64, t11262: f64, t4802: f64, t4807: f64, t11859: f64, t11922: f64, t15894: f64, t11714: f64, t4817: f64, t12004: f64, t3299: f64, t53401: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55061 = t1063 * t11262 * t4802;
    let t55064 = t1063 * t11262 * t4807;
    let t55067 = t11859 * t11922 * t15894;
    let t55070 = t11714 * t4817;
    let t55072 = t12004 * t4817;
    let t55100 = t3299 * t53401;
    (t55061, t55064, t55067, t55070, t55072, t55100)
}
