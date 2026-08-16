//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1019/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1019(t10658: f64, t19273: f64, t19276: f64, t19278: f64, t19283: f64, t19287: f64, t19292: f64, t19295: f64, t19298: f64, t19301: f64, t19304: f64, t19737: f64, t19748: f64, t19769: f64) -> f64 {
    let t19780 = 2.0_f64 / 27.0_f64 * t19273 + 4.0_f64 / 9.0_f64 * t19276 - 2.0_f64 / 27.0_f64 * t19278 - t10658 + 2.0_f64 / 3.0_f64 * t19283 - t19287 / 9.0_f64 - 2.0_f64 * t19292 + 4.0_f64 / 3.0_f64 * t19295 + t19298 / 27.0_f64 - 2.0_f64 / 27.0_f64 * t19301 + 2.0_f64 / 81.0_f64 * t19304;
    let t19782 = t19737 + t19748 + t19769 + t19780;
    t19782
}
