//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 258/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk258(t898: f64, t900: f64, t904: f64, t631: f64, t892: f64, t895: f64, t332: f64, t113: f64, t19: f64, t362: f64) -> (f64, f64, f64, f64, f64) {
    let t906 = t898 * t900 * t904;
    let t909 = t892 + t631 * t895 / 6.0_f64 + t631 * t906 / 2.0_f64;
    let t910 = t909 * t332;
    let t911 = t910 * t113;
    let t920 = -t19 - t362;
    (t906, t909, t910, t911, t920)
}
