//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 989/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk989(t1636: f64, t7386: f64, t89: f64, t7382: f64, t1557: f64, t7339: f64, t1882: f64, t32981: f64, t32869: f64, t358: f64, t1570: f64, t32964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t139492 = t89 * t1636 * t7386;
    let t139493 = 4.0_f64 / 27.0_f64 * t139492;
    let t139495 = t89 * t1636 * t7382;
    let t139496 = 8.0_f64 / 27.0_f64 * t139495;
    let t139497 = t7339 * t1557;
    let t139507 = t1882 * t32981;
    let t139509 = t32869 * t358;
    let t139514 = t7339 * t1570;
    let t139519 = t1882 * t32964;
    (t139492, t139493, t139495, t139496, t139497, t139507, t139509, t139514, t139519)
}
