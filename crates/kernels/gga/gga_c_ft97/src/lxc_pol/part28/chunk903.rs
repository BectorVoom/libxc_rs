//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 903/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk903<F: Float>(t144998: F, t7238: F, t7239: F, t7243: F, t28: F, t32325: F, t469: F, t5665: F, t965: F, t3157: F, t7211: F, t144893: F, t446: F, t7824: F, t144846: F, t38262: F) -> (F, F, F, F, F) {
    let t145012 = t7238 * t7239 * t7243 * t144998;
    let t145017 = t5665 * t28 * t469 * t32325 * t965;
    let t145022 = t5665 * t28 * t469 * t7211 * t3157;
    let t145025 = t446 * t7824 * t144893;
    let t145028 = t446 * t38262 * t144846;
    (t145012, t145017, t145022, t145025, t145028)
}
