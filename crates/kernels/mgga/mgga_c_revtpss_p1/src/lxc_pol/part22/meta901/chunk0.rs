//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3095/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3095<F: Float>(t16094: F, t53884: F, t11922: F, t16021: F, t4899: F, t3091: F, t43240: F, t4787: F, t12160: F, t15688: F, t1087: F, t43065: F) -> (F, F, F, F, F) {
    let t53885 = t16094 * t53884;
    let t53898 = t4899 * t11922 * t16021;
    let t53901 = t3091 * t43240 * t4787;
    let t53914 = t12160 * t15688;
    let t53923 = t1087 * t43065;
    (t53885, t53898, t53901, t53914, t53923)
}
