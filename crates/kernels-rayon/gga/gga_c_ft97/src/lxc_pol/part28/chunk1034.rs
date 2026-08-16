//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1034/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1034(t28: f64, t3157: f64, t469: f64, t5665: f64, t7211: f64, t144893: f64, t446: f64, t7824: f64, t144846: f64, t38262: f64, t144938: f64, t32057: f64, t7239: f64, t7243: f64) -> (f64, f64, f64, f64) {
    let t145022 = t5665 * t28 * t469 * t7211 * t3157;
    let t145025 = t446 * t7824 * t144893;
    let t145028 = t446 * t38262 * t144846;
    let t145032 = t32057 * t7239 * t7243 * t144938;
    (t145022, t145025, t145028, t145032)
}
