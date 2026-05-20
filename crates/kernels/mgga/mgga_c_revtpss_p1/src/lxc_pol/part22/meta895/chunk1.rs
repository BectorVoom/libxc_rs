//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3087/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3087<F: Float>(t1011: F, t4886: F, t697: F, t1065: F, t372: F, t4866: F, t11670: F, t15904: F, t12167: F, t11922: F, t16081: F, t16083: F) -> (F, F, F, F, F) {
    let t53542 = t1011 * t697 * t4886;
    let t53545 = t372 * t1065 * t4866;
    let t53552 = t11670 * t15904;
    let t53553 = t12167 * t53552;
    let t53557 = t16081 * t11922 * t16083;
    (t53542, t53545, t53552, t53553, t53557)
}
