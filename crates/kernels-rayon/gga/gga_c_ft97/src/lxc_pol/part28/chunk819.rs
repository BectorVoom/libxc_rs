//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 819/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk819(t379: f64, t7357: f64, t9144: f64, t167: f64, t2185: f64, t32951: f64, t609: f64, t7339: f64, t574: f64, t605: f64, t558: f64, t7400: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33067 = t7357 * t379;
    let t33068 = t9144 * t33067;
    let t33072 = t2185 * t167 * t32951;
    let t33075 = t7339 * t609;
    let t33077 = t574 * t605 * t33075;
    let t33080 = t7400 * t558;
    (t33067, t33068, t33072, t33075, t33077, t33080)
}
