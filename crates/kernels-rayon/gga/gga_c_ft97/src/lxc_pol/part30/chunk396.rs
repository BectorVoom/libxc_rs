//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 396/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk396(t6386: f64, t871: f64, t296: f64, t1901: f64, t193: f64, t446: f64, t6272: f64, t6275: f64, t6280: f64, t6284: f64, t6289: f64, t6293: f64, t6298: f64, t6300: f64, t6304: f64, t6349: f64, t6355: f64, t6359: f64, t6362: f64, t6367: f64, t6371: f64, t6376: f64, t89: f64) -> (f64, f64) {
    let t6387 = t871 * t6386;
    let t6388 = t296 * t6387;
    let t6391 = t6272 + t1901 * t6275 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6280 - t446 * t6284 / 3.0_f64 + t446 * t6289 / 3.0_f64 - t446 * t6293 / 3.0_f64 - t6298 - t446 * t6300 / 9.0_f64 - t446 * t6304 / 3.0_f64 + t89 * t193 * t6349 / 3.0_f64 - t446 * t6355 / 3.0_f64 + t6359 + t1901 * t6362 / 9.0_f64 + t446 * t6367 / 3.0_f64 - t446 * t6371 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6376 - t446 * t6388 / 3.0_f64;
    (t6388, t6391)
}
