//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 188/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk188(t605: f64, t609: f64, t144: f64, t28: f64, t446: f64, t568: f64, t571: f64, t576: f64, t599: f64, t89: f64) -> (f64, f64, f64) {
    let t610 = t605 * t609;
    let t611 = t144 * t610;
    let t614 = -t568 - t446 * t571 / 9.0_f64 - t446 * t576 / 3.0_f64 + t89 * t28 * t599 / 3.0_f64 - t446 * t611 / 3.0_f64;
    (t610, t611, t614)
}
