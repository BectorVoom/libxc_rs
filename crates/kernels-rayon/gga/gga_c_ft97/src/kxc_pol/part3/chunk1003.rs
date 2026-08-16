//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1003/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1003(t19526: f64, t2882: f64, t2881: f64, t11593: f64, t1901: f64, t19479: f64, t19482: f64, t19484: f64, t19487: f64, t19491: f64, t19494: f64, t19497: f64, t19501: f64, t19504: f64, t19508: f64, t19511: f64, t19514: f64, t19519: f64, t19523: f64) -> f64 {
    let t19527 = t2882 * t19526;
    let t19528 = t2881 * t19527;
    let t19531 = 2.0_f64 / 9.0_f64 * t1901 * t19479 - 2.0_f64 / 9.0_f64 * t19482 - 2.0_f64 / 9.0_f64 * t19484 + 4.0_f64 / 9.0_f64 * t1901 * t19487 - 10.0_f64 / 81.0_f64 * t1901 * t19491 + 2.0_f64 / 9.0_f64 * t1901 * t19494 + 4.0_f64 / 9.0_f64 * t1901 * t19497 - 4.0_f64 / 27.0_f64 * t1901 * t19501 - 2.0_f64 / 27.0_f64 * t19504 - 4.0_f64 / 3.0_f64 * t1901 * t19508 - 2.0_f64 / 27.0_f64 * t19511 + 8.0_f64 / 27.0_f64 * t11593 * t19514 - 2.0_f64 / 9.0_f64 * t1901 * t19519 - 4.0_f64 / 3.0_f64 * t1901 * t19523 + t1901 * t19528 / 9.0_f64;
    t19531
}
