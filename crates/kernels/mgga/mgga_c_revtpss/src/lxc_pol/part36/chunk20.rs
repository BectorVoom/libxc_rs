//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 20/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk20<F: Float>(t45: F, rho1: F) -> (F, F, F, F, F, F) {
    let t46 = t45 / F::new(2.0);
    let t47 = pow_1_3::<F>(t46);
    let t48 = t47 * t47;
    let t49 = t48 * t46;
    let t51 = rho1 * rho1;
    let t52 = pow_1_3::<F>(rho1);
    (t46, t47, t48, t49, t51, t52)
}
