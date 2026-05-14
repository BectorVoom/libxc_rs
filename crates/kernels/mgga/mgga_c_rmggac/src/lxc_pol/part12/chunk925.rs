//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 925/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk925<F: Float>(t275: F, t9031: F, t40884: F, t7204: F, t118: F, t2281: F, t498: F, t7418: F, t7720: F, t236: F, t495: F, t7230: F, t7248: F, t9216: F, t7244: F, t9153: F) -> (F, F, F, F, F) {
    let t41905 = 2.0 * t275 * t9031;
    let t41906 = t7204 * t40884;
    let t41914 = t7418 * t118 * t2281 * t498;
    let t41915 = t7720 * t41914;
    let t41920 = t7230 * t7248 * t236 * t9216 * t495;
    let t41922 = t7244 * t9153;
    (t41905, t41906, t41915, t41920, t41922)
}
