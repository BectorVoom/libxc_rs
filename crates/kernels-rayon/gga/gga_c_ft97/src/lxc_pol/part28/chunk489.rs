//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 489/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk489(t605: f64, t7407: f64, t144: f64, t28: f64, t446: f64, t7350: f64, t7354: f64, t7359: f64, t7363: f64, t7392: f64, t7397: f64, t7402: f64, t89: f64) -> (f64, f64, f64) {
    let t7408 = t605 * t7407;
    let t7409 = t144 * t7408;
    let t7412 = 2.0_f64 / 3.0_f64 * t446 * t7350 - 2.0_f64 / 3.0_f64 * t446 * t7354 + 2.0_f64 / 3.0_f64 * t446 * t7359 - t446 * t7363 / 3.0_f64 + t89 * t28 * t7392 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t7397 + 2.0_f64 / 3.0_f64 * t446 * t7402 - t446 * t7409 / 3.0_f64;
    (t7408, t7409, t7412)
}
