//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1159/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1159(t148270: f64, t148275: f64, t148278: f64, t148282: f64, t148286: f64, t148290: f64, t148295: f64, t148299: f64, t148304: f64, t148309: f64, t148311: f64, t148315: f64, t148319: f64, t148323: f64, t148327: f64, t148331: f64) -> f64 {
    let t148750 = -6.0_f64 * t148270 - 15.0_f64 / 4.0_f64 * t148275 - t148278 / 3.0_f64 - t148282 - t148286 - 2.0_f64 / 3.0_f64 * t148290 - 20.0_f64 * t148295 + 8.0_f64 * t148299 + 4.0_f64 / 3.0_f64 * t148304 + t148309 / 6.0_f64 + t148311 / 9.0_f64 + 3.0_f64 / 2.0_f64 * t148315 - t148319 / 12.0_f64 + 8.0_f64 * t148323 + t148327 / 3.0_f64 - t148331 / 2.0_f64;
    t148750
}
