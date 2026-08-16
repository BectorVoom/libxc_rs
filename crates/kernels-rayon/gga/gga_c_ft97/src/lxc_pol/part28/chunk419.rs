//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 419/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk419(t452: f64, t488: f64, t6478: f64, t110: f64, t6454: f64, t1339: f64, t447: f64, t925: f64, t942: f64, t1307: f64, t965: f64, t469: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6480 = t452 * t488 * t6478;
    let t6484 = t452 * t110 * t6454;
    let t6488 = t447 * t1339 * t925;
    let t6492 = t452 * t1339 * t942;
    let t6495 = t1307 * t965;
    let t6496 = t469 * t6495;
    (t6480, t6484, t6488, t6492, t6495, t6496)
}
