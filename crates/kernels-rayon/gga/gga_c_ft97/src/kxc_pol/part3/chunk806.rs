//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 806/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk806(t103: f64, t16533: f64, t82: f64, t1882: f64, t4569: f64, t4595: f64, t11897: f64, t11913: f64, t11981: f64, t11999: f64, t12002: f64, t16482: f64, t16486: f64, t16490: f64, t1901: f64, t28: f64, t446: f64, t8475: f64, t8485: f64, t8516: f64, t8534: f64, t89: f64) -> f64 {
    let t16535 = t82 * t16533 * t103;
    let t16539 = t1882 * t4569;
    let t16541 = t1882 * t4595;
    let t16544 = -4.0_f64 / 27.0_f64 * t8475 - 4.0_f64 / 27.0_f64 * t8485 - t446 * t16482 / 3.0_f64 - t11897 - 2.0_f64 / 9.0_f64 * t1901 * t16486 - t11913 + 4.0_f64 / 27.0_f64 * t8516 - t8534 - 2.0_f64 / 9.0_f64 * t16490 + t89 * t28 * t16535 / 3.0_f64 - t11981 - 2.0_f64 / 9.0_f64 * t16539 + 2.0_f64 / 9.0_f64 * t16541 - t11999 + 4.0_f64 / 27.0_f64 * t12002;
    t16544
}
