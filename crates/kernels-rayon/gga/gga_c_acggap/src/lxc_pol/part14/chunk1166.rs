//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1166/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1166(t31142: f64, t9727: f64, t2060: f64, t361: f64, t9733: f64, t7450: f64, t9659: f64, t13287: f64, t31195: f64, t38861: f64, t13364: f64, t38850: f64) -> (f64, f64, f64, f64, f64) {
    let t40083 = t31142 * t9727;
    let t40086 = t2060 * t361 * t9733;
    let t40089 = t7450 * t361 * t9659;
    let t40092 = t31195 * t13287 * t38861;
    let t40095 = t31195 * t13364 * t38850;
    (t40083, t40086, t40089, t40092, t40095)
}
