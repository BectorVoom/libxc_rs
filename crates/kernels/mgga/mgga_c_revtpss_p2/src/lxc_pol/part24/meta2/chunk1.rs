//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 21/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk21<F: Float>(rho0: F, sigma0: F) -> (F, F, F, F) {
    let t39 = rho0 * rho0;
    let t40 = pow_1_3::<F>(rho0);
    let t41 = t40 * t40;
    let t43 = F::new(1.0) / t41 / t39;
    let t44 = sigma0 * t43;
    (t39, t40, t41, t44)
}
