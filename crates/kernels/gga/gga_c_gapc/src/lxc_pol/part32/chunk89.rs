//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 89/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk89<F: Float>(t62: F, t80: F, t85: F, t88: F, t97: F) -> (F,) {
    let t266 = -0.77371026992393176896e-2 * t62 + 0.187495875e-2 * t80 - 0.362780625e-3 * t85 + 0.10208501871552144532e-4 * t88 - 0.8659659375e-6 * t97;
    (t266,)
}
