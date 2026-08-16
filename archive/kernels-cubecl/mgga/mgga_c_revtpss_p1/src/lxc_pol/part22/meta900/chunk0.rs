//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3093/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3093<F: Float>(t11710: F, t16089: F, t16090: F, t11883: F, t4924: F, t1086: F, t15654: F, t3090: F, t11922: F, t16077: F, t3115: F, t225: F, t53222: F) -> (F, F, F, F, F) {
    let t53820 = t16089 * t11710 * t16090;
    let t53832 = t11883 * t4924;
    let t53855 = t15654 * t1086 * t3090;
    let t53859 = t3115 * t11922 * t16077;
    let t53865 = t53222 * t225;
    (t53820, t53832, t53855, t53859, t53865)
}
