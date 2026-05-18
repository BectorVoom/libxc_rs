//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 833/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk833<F: Float>(t1901: F, t19449: F, t19451: F, t19453: F, t19482: F, t19484: F, t19504: F, t19511: F, t22218: F, t22222: F, t22226: F, t22230: F, t22242: F, t22246: F, t22251: F, t22255: F, t446: F) -> F {
    let t22258 = -t446 * t22218 / F::new(9.0) - F::new(10.0) / F::new(81.0) * t446 * t22222 - t446 * t22226 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t446 * t22230 + F::new(2.0) / F::new(3.0) * t19449 + t19451 / F::new(3.0) + t19453 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t19482 - F::new(2.0) / F::new(3.0) * t19484 - F::new(2.0) / F::new(9.0) * t19504 - F::new(2.0) / F::new(9.0) * t19511 - F::new(2.0) / F::new(3.0) * t1901 * t22242 - F::new(2.0) / F::new(3.0) * t1901 * t22246 - F::new(2.0) * t446 * t22251 + F::new(4.0) / F::new(9.0) * t446 * t22255;
    t22258
}
