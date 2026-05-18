//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 469/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk469<F: Float>(t7553: F, t762: F, t242: F, t193: F, t446: F, t7495: F, t7499: F, t7504: F, t7508: F, t7538: F, t7543: F, t7548: F, t89: F) -> (F, F, F) {
    let t7554 = t762 * t7553;
    let t7555 = t242 * t7554;
    let t7558 = F::new(2.0) / F::new(3.0) * t446 * t7495 - F::new(2.0) / F::new(3.0) * t446 * t7499 + F::new(2.0) / F::new(3.0) * t446 * t7504 - t446 * t7508 / F::new(3.0) + t89 * t193 * t7538 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t7543 + F::new(2.0) / F::new(3.0) * t446 * t7548 - t446 * t7555 / F::new(3.0);
    (t7554, t7555, t7558)
}
