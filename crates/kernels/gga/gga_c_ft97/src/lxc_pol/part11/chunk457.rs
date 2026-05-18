//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 457/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk457<F: Float>(t2481: F, t2482: F, t2484: F, t2489: F, t2494: F, t2499: F, t2503: F, t2508: F, t2512: F, t462: F, t92: F, t734: F, t91: F) -> (F, F) {
    let t2514 = t2481 + F::new(2.0) / F::new(9.0) * t2482 + F::new(2.0) / F::new(3.0) * t2484 - F::new(2.0) / F::new(9.0) * t462 * t2489 + F::new(2.0) / F::new(3.0) * t462 * t2494 + F::new(2.0) / F::new(3.0) * t462 * t2499 - t462 * t2503 / F::new(3.0) + F::new(2.0) * t92 * t2508 - t92 * t2512;
    let t2516 = t91 * t734 * t2514;
    (t2514, t2516)
}
