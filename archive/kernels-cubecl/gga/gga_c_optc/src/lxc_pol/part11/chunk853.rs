//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 853/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk853<F: Float>(t16416: F, t16438: F, t16464: F, t16471: F, t16474: F, t16479: F, t16483: F, t16487: F, t16490: F, t16493: F, t16496: F, t2113: F, t2159: F, t673: F, t686: F, t695: F, t6993: F, t7002: F, t705: F) -> F {
    let t16499 = -F::cast_from(0.20863587575493018851e1_f64) * t686 * t16464 - F::cast_from(0.30228422675018518372e-1_f64) * t705 * t16438 - F::cast_from(0.90685268025055555116e0_f64) * t705 * t16416 + F::cast_from(0.13602790203758333267e0_f64) * t2159 * t16471 - F::cast_from(0.18137053605011111023e0_f64) * t6993 * t16474 - F::cast_from(0.52158968938732547127e0_f64) * t7002 * t16479 + F::cast_from(0.52158968938732547127e0_f64) * t2113 * t16483 - F::cast_from(0.86931614897887578546e-1_f64) * t673 * t16487 + F::cast_from(0.45342634012527777558e0_f64) * t705 * t16490 + F::cast_from(0.15647690681619764138e1_f64) * t686 * t16493 - F::cast_from(0.15114211337509259186e-1_f64) * t695 * t16496;
    t16499
}
