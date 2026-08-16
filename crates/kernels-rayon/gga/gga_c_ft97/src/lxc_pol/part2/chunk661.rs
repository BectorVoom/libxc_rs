//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 661/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk661(t231: f64, t2459: f64, t1526: f64, t2320: f64, t2331: f64, t2355: f64, t2465: f64, t342: f64, t343: f64, t3806: f64, t9482: f64, t9485: f64, t9488: f64, t9491: f64, t9499: f64, t9503: f64) -> f64 {
    let t9507 = t231 * t2459;
    let t9511 = t2331 + t2465 + t9482 - t9485 / 18.0_f64 - t9488 / 6.0_f64 - t1526 * t3806 * t9491 / 9.0_f64 - t1526 * t2320 * t2355 / 6.0_f64 + t1526 * t2320 * t9499 / 6.0_f64 - t1526 * t2320 * t9503 / 12.0_f64 - t342 * t343 * t9507 / 4.0_f64;
    t9511
}
