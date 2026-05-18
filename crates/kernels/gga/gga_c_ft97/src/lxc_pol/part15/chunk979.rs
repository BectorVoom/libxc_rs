//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 979/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk979<F: Float>(t21875: F, t8675: F, t13598: F, t1526: F, t21911: F, t5213: F, t9483: F, t21922: F, t21918: F, t21926: F, t342: F, t630: F) -> (F, F, F, F, F, F) {
    let t82409 = t8675 * t21875;
    let t82488 = t1526 * t13598 * t21911;
    let t82491 = t1526 * t9483 * t5213;
    let t82494 = t1526 * t9483 * t21922;
    let t82497 = t1526 * t9483 * t21918;
    let t82552 = t342 * t630 * t21926;
    (t82409, t82488, t82491, t82494, t82497, t82552)
}
