//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 22/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk22<F: Float>(t11: F, t14: F) -> (F, F, F, F) {
    let t65 = F::new(0.107924e1) + F::new(0.3964e-1) * t14 + F::new(0.123825e-1) * t11;
    let t68 = F::new(1.0) + t14 * t65 / F::new(2.0);
    let t69 = t68 * t68;
    let t70 = F::new(1.0) / t69;
    (t65, t68, t69, t70)
}
