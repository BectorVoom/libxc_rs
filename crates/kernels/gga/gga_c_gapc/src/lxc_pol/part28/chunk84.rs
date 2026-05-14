//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 84/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk84<F: Float>(t179: F, t182: F, t192: F, t205: F, t62: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t208 = 1.0 + 0.13900948042322754167e-2 * t179 * t182 - 0.57970906942607043474e-5 * t192 * t205;
    let t209 = 1.0 / t208;
    let t211 = rho0 - rho1;
    let t212 = t211 * t62;
    let t213 = 1.0 + t212;
    let t214 = t213 <= zeta_threshold;
    let t215 = pow_1_3(t213);
    (t208, t209, t211, t212, t213, t215)
}
