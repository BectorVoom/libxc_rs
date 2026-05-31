//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 716/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk716<F: Float>(t1943: F, t4431: F, t4714: F, t72: F, t1526: F, t1527: F, t16631: F, t16649: F, t20507: F, t20514: F, t3088: F, t342: F, t343: F, t4650: F, t4656: F, t4720: F, t8759: F) -> (F, F, F) {
    let t20518 = t1943 * t4431;
    let t20522 = t72 * t4714;
    let t20526 = t4650 + t4720 + t8759 - t16631 / F::cast_from(18.0_f64) - t16649 / F::cast_from(6.0_f64) - t1526 * t3088 * t20507 / F::cast_from(9.0_f64) - t1526 * t1527 * t4656 / F::cast_from(6.0_f64) + t1526 * t1527 * t20514 / F::cast_from(6.0_f64) - t1526 * t1527 * t20518 / F::cast_from(12.0_f64) - t342 * t343 * t20522 / F::cast_from(4.0_f64);
    (t20518, t20522, t20526)
}
