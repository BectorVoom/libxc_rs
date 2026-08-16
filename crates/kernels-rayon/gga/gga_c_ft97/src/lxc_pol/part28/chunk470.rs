//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 470/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk470(t488: f64, t7281: f64, t83: f64, t28: f64, t446: f64, t7222: f64, t7226: f64, t7231: f64, t7235: f64, t7266: f64, t7271: f64, t7276: f64, t89: f64) -> (f64, f64, f64) {
    let t7282 = t488 * t7281;
    let t7283 = t83 * t7282;
    let t7286 = 2.0_f64 / 3.0_f64 * t446 * t7222 - 2.0_f64 / 3.0_f64 * t446 * t7226 + 2.0_f64 / 3.0_f64 * t446 * t7231 - t446 * t7235 / 3.0_f64 + t89 * t28 * t7266 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t7271 + 2.0_f64 / 3.0_f64 * t446 * t7276 - t446 * t7283 / 3.0_f64;
    (t7282, t7283, t7286)
}
