//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3119/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3119<F: Float>(t1063: F, t11262: F, t4802: F, t4807: F, t11859: F, t11922: F, t15894: F, t11714: F, t4817: F, t12004: F, t3299: F, t53401: F) -> (F, F, F, F, F, F) {
    let t55061 = t1063 * t11262 * t4802;
    let t55064 = t1063 * t11262 * t4807;
    let t55067 = t11859 * t11922 * t15894;
    let t55070 = t11714 * t4817;
    let t55072 = t12004 * t4817;
    let t55100 = t3299 * t53401;
    (t55061, t55064, t55067, t55070, t55072, t55100)
}
