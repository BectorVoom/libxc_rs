//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 868/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk868(t144: f64, t17500: f64, t13196: f64, t13201: f64, t17426: f64, t17429: f64, t17432: f64, t17434: f64, t17436: f64, t17438: f64, t17440: f64, t17443: f64, t17488: f64, t17493: f64, t17497: f64, t1901: f64, t28: f64, t446: f64, t89: f64, t9457: f64) -> f64 {
    let t17501 = t144 * t17500;
    let t17504 = t13196 - 4.0_f64 / 9.0_f64 * t1901 * t17426 + 4.0_f64 / 27.0_f64 * t1901 * t17429 - 2.0_f64 / 27.0_f64 * t17432 + 2.0_f64 / 81.0_f64 * t17434 + t17436 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t17438 + 2.0_f64 / 27.0_f64 * t17440 - t17443 / 9.0_f64 + t89 * t28 * t17488 / 3.0_f64 - 8.0_f64 / 27.0_f64 * t13201 - t9457 - 4.0_f64 / 9.0_f64 * t1901 * t17493 - 2.0_f64 / 9.0_f64 * t1901 * t17497 - t446 * t17501 / 3.0_f64;
    t17504
}
