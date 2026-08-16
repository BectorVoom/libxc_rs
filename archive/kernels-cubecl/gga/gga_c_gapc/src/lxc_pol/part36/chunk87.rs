//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 87/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk87<F: Float>(t62: F, t80: F, t85: F, t88: F, t97: F) -> F {
    let t266 = -F::cast_from(0.77371026992393176896e-2_f64) * t62 + F::cast_from(0.187495875e-2_f64) * t80 - F::cast_from(0.362780625e-3_f64) * t85 + F::cast_from(0.10208501871552144532e-4_f64) * t88 - F::cast_from(0.8659659375e-6_f64) * t97;
    t266
}
