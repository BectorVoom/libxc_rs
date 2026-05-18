//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 836/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk836<F: Float>(t19649: F, t19651: F, t19653: F, t19691: F, t19693: F, t19695: F, t22284: F, t22287: F, t22291: F, t22294: F, t22298: F, t462: F, t92: F) -> F {
    let t22301 = -F::new(2.0) / F::new(3.0) * t19691 + t19693 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t19695 + t19651 - F::new(2.0) * t19653 + F::new(6.0) * t462 * t22284 - t462 * t22287 / F::new(3.0) - F::new(6.0) * t92 * t22291 - F::new(10.0) / F::new(27.0) * t462 * t22294 - F::new(2.0) / F::new(3.0) * t19649 - F::new(2.0) * t462 * t22298;
    t22301
}
