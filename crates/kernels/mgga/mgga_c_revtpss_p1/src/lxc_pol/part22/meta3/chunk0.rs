//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 24/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk24<F: Float>(t46: F, t48: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t49 = t48 * t46;
    let t51 = rho1 * rho1;
    let t52 = pow_1_3::<F>(rho1);
    let t53 = t52 * t52;
    let t55 = F::new(1.0) / t53 / t51;
    let t56 = sigma2 * t55;
    (t49, t51, t52, t53, t55, t56)
}
