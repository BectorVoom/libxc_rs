//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 596/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk596<F: Float>(t1806: F, t458: F, t462: F, t8263: F, t8267: F, t8272: F, t8278: F, t8283: F, t8285: F, t8287: F, t8289: F, t8292: F, t8295: F, t92: F) -> F {
    let t8298 = t458 * t1806;
    let t8299 = F::new(6.0) * t462 * t8263 - t462 * t8267 / F::new(3.0) - F::new(6.0) * t92 * t8272 - F::new(10.0) / F::new(27.0) * t462 * t8278 - F::new(4.0) / F::new(9.0) * t8283 + t8285 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t8287 - F::new(2.0) * t8289 - F::new(2.0) * t462 * t8292 - F::new(2.0) * t462 * t8295 + t8298;
    t8299
}
