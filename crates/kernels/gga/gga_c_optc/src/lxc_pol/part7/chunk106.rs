//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 106/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk106<F: Float>(t216: F, t232: F, t46: F, t56: F, t59: F, zeta_threshold: F) -> (F, F) {
    let t234 = F::new(0.62182e-1) * t216 * t232;
    let t235 = F::new(2.0) <= zeta_threshold;
    let t237 = piecewise3::<F>(t235, t46, F::new(2.0) * t56);
    let t238 = F::new(0.0) <= zeta_threshold;
    let t239 = piecewise3::<F>(t238, t46, F::new(0.0));
    let t241 = (t237 + t239 - F::new(2.0)) * t59;
    (t234, t241)
}
