//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 675/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk675(t9059: f64, t9071: f64, t9062: f64, t9014: f64, t9024: f64, t9028: f64, t9032: f64, t9057: f64, t9076: f64, t9080: f64, t9170: f64, t9245: f64, t9255: f64) -> f64 {
    let t9380 = 2.0_f64 / 9.0_f64 * t9059;
    let t9383 = 28.0_f64 / 81.0_f64 * t9071;
    let t9390 = 2.0_f64 / 9.0_f64 * t9062;
    let t9393 = -t9380 - 2.0_f64 / 3.0_f64 * t9076 - 2.0_f64 / 3.0_f64 * t9080 - t9383 - t9014 / 9.0_f64 - t9170 / 4.0_f64 + 2.0_f64 * t9024 - 10.0_f64 / 81.0_f64 * t9028 - 2.0_f64 / 3.0_f64 * t9032 + 4.0_f64 / 9.0_f64 * t9057 - t9390 + t9245 / 6.0_f64 + t9255 / 8.0_f64;
    t9393
}
