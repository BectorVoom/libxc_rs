//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 932/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk932(t11402: f64, t2437: f64, t13261: f64, t1572: f64, t4673: f64, t11485: f64, t3377: f64, t3566: f64, t9333: f64, t2365: f64, t35913: f64, t4391: f64) -> (f64, f64, f64, f64, f64) {
    let t46767 = 0.35750489951850426669e0_f64 * t2437 * t11402;
    let t46773 = 0.47667319935800568892e0_f64 * t1572 * t4673 * t13261;
    let t46775 = 0.25025342966295298669e1_f64 * t11485 * t3377;
    let t46778 = 0.25025342966295298669e1_f64 * t3566 * t9333;
    let t46784 = t4391 * t2365 * t35913;
    (t46767, t46773, t46775, t46778, t46784)
}
