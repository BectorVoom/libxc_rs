//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 877/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk877(t10188: f64, t13699: f64, t13701: f64, t13703: f64, t16630: f64, t16634: f64, t16638: f64, t16642: f64, t16646: f64, t16650: f64, t7699: f64, t1354: f64, t4868: f64) -> (f64, f64) {
    let t16699 = -t7699 - 0.2283111111111111111e-1_f64 * t10188 + 0.11415555555555555555e-1_f64 * t13699 - 0.34246666666666666665e-1_f64 * t13701 + 0.17123333333333333333e-1_f64 * t13703 - 0.19025925925925925925e-1_f64 * t16630 + 0.68493333333333333331e-1_f64 * t16634 - 0.34246666666666666665e-1_f64 * t16638 - 0.10274e0_f64 * t16642 + 0.10274e0_f64 * t16646 - 0.17123333333333333333e-1_f64 * t16650;
    let t16708 = t4868 * t1354;
    (t16699, t16708)
}
