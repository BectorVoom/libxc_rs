//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 782/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk782(t21443: f64, t446: f64, t21181: f64, t9749: f64, t2345: f64, t89: f64, t21196: f64, t2594: f64, t1131: f64, t4973: f64, t2354: f64, t1091: f64, t5053: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21444 = t446 * t21443;
    let t21446 = t9749 * t21181;
    let t21448 = t89 * t2345 * t21446;
    let t21450 = t2594 * t21196;
    let t21451 = t446 * t21450;
    let t21453 = t4973 * t1131;
    let t21454 = t2354 * t21453;
    let t21455 = t446 * t21454;
    let t21457 = t1091 * t5053;
    (t21444, t21446, t21448, t21450, t21451, t21453, t21454, t21455, t21457)
}
