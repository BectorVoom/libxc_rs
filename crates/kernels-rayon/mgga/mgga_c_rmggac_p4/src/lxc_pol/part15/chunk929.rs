//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 929/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk929(t10014: f64, t35637: f64, t34884: f64, t9990: f64, t10095: f64, t16156: f64, t10082: f64, t333: f64, t3351: f64, t511: f64, t7248: f64, t38530: f64, t9159: f64) -> (f64, f64, f64, f64, f64) {
    let t45484 = t35637 * t10014;
    let t45486 = t34884 * t9990;
    let t45488 = t16156 * t10095;
    let t45493 = t3351 * t7248 * t511 * t10082 * t333;
    let t45495 = t38530 * t9159;
    (t45484, t45486, t45488, t45493, t45495)
}
