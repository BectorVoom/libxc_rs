//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3085/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3085<F: Float>(t15689: F, t15692: F, t53405: F, t11916: F, t15932: F, t11922: F, t11927: F, t16026: F, t11710: F, t15964: F, t3091: F, t11268: F, t4820: F) -> (F, F, F, F, F) {
    let t53407 = t15689 * t53405 * t15692;
    let t53413 = t15932 * t11916;
    let t53416 = t11927 * t11922 * t16026;
    let t53422 = t3091 * t11710 * t15964;
    let t53427 = t11268 * t4820;
    (t53407, t53413, t53416, t53422, t53427)
}
