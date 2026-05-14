//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 225/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk225<F: Float>(t512: F, t537: F, t541: F, t546: F, t560: F, t593: F, t595: F, t600: F, t605: F, t120: F, t138: F, t124: F, t616: F, t121: F) -> (F, F, F, F) {
    let t637 = t512 + t537 + t541 - t546 + t560 + t593 + t595 - t600 - t605;
    let t641 = t120 * t138;
    let t642 = t124 * t616;
    let t645 = -0.12897460341341234505e3 * t637 * t121 * t124 + 0.38692381024023703515e3 * t641 * t642;
    (t637, t641, t642, t645)
}
