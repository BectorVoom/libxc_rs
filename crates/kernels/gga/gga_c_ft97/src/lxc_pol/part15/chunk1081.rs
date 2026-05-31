//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1081/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1081<F: Float>(t20972: F, t3578: F, t1526: F, t1527: F, t1943: F, t20044: F, t20527: F, t20529: F, t20655: F, t20678: F, t342: F, t343: F, t41328: F, t64663: F, t64677: F, t72: F, t78650: F, t78653: F, t78700: F) -> (F, F) {
    let t87220 = t3578 * t20972;
    let t87252 = t78650 / F::cast_from(6.0_f64) + t64677 / F::cast_from(6.0_f64) - t78653 / F::cast_from(12.0_f64) + t20527 + t20678 - t41328 + F::cast_from(2.0_f64) * t20529 - t342 * t343 * t72 * t20655 / F::cast_from(4.0_f64) - t1526 * t1527 * t1943 * t20044 / F::cast_from(12.0_f64) + t64663 / F::cast_from(18.0_f64) - t78700 / F::cast_from(4.0_f64);
    (t87220, t87252)
}
