//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 13/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk13<F: Float>(t21: F, t22: F, t5: F, t11: F, t14: F, t17: F, t13: F, rho0: F, rho1: F) -> (F, F, F, F, F, F) {
    let t25 = t21 * t5 / t22;
    let t27 = F::new(0.379785e1) * t14 + F::new(0.8969e0) * t11 + F::new(0.204775e0) * t17 + F::new(0.123235e0) * t25;
    let t30 = F::new(1.0) + F::new(0.16081979498692535067e2) / t27;
    let t31 = f64::ln(t30);
    let t33 = F::new(0.621814e-1) * t13 * t31;
    let t34 = rho0 - rho1;
    (t25, t27, t30, t31, t33, t34)
}
