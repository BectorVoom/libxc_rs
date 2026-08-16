//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 988/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk988(t2336: f64, t5217: f64, t89: f64, t5209: f64, t9725: f64, t10398: f64, t19243: f64, t19246: f64, t19249: f64, t19252: f64, t19255: f64, t19258: f64, t19261: f64, t19265: f64, t19269: f64, t19273: f64, t19276: f64, t19278: f64, t19283: f64, t19287: f64, t19292: f64, t19295: f64, t19298: f64) -> (f64, f64, f64) {
    let t19301 = t89 * t2336 * t5217;
    let t19304 = t89 * t9725 * t5209;
    let t19306 = -t19243 / 6.0_f64 + t19246 / 18.0_f64 - t19249 / 9.0_f64 + t19252 / 9.0_f64 - t19255 / 27.0_f64 - 5.0_f64 / 81.0_f64 * t19258 + 4.0_f64 / 27.0_f64 * t19261 + t19265 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t19269 + t19273 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t19276 - t19278 / 27.0_f64 - t10398 + t19283 / 3.0_f64 - t19287 / 18.0_f64 - t19292 + 2.0_f64 / 3.0_f64 * t19295 + t19298 / 54.0_f64 - t19301 / 27.0_f64 + t19304 / 81.0_f64;
    (t19301, t19304, t19306)
}
