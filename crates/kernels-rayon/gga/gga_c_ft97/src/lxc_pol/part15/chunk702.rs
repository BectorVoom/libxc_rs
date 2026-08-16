//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 702/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk702(t4505: f64, t965: f64, t8345: f64, t91: f64, t20098: f64, t24: f64, t469: f64, t20044: f64, t464: f64, t463: f64, t20113: f64, t8270: f64) -> (f64, f64, f64, f64, f64) {
    let t20329 = t4505 * t965;
    let t20331 = t91 * t8345 * t20329;
    let t20334 = t24 * t469 * t20098;
    let t20336 = t464 * t20044;
    let t20337 = t463 * t20336;
    let t20341 = t24 * t8270 * t20113;
    (t20331, t20334, t20336, t20337, t20341)
}
