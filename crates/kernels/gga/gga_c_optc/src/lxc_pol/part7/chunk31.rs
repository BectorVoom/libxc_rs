//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 31/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk31<F: Float>(t50: F, t52: F, t46: F, t49: F, zeta_threshold: F) -> (F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t53 = t52 * t50;
    let t54 = piecewise3::<f64>(t51, t46, t53);
    let t55 = t49 + t54 - F::new(2.0);
    let t56 = M_CBRT2;
    (t53, t55, t56)
}
