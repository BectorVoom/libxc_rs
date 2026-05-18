//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 661/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk661<F: Float>(t231: F, t2459: F, t1526: F, t2320: F, t2331: F, t2355: F, t2465: F, t342: F, t343: F, t3806: F, t9482: F, t9485: F, t9488: F, t9491: F, t9499: F, t9503: F) -> F {
    let t9507 = t231 * t2459;
    let t9511 = t2331 + t2465 + t9482 - t9485 / F::new(18.0) - t9488 / F::new(6.0) - t1526 * t3806 * t9491 / F::new(9.0) - t1526 * t2320 * t2355 / F::new(6.0) + t1526 * t2320 * t9499 / F::new(6.0) - t1526 * t2320 * t9503 / F::new(12.0) - t342 * t343 * t9507 / F::new(4.0);
    t9511
}
