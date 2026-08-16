//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1013/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1013(t7433: f64, t8787: f64, t31362: f64, t8956: f64, t7839: f64, t8962: f64, t8966: f64, t33953: f64, t5284: f64, t13299: f64, t31115: f64, t31276: f64, t8875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35610 = t7433 * t8787;
    let t35616 = t31362 * t8956;
    let t35623 = t7839 * t8962;
    let t35631 = t7839 * t8966;
    let t35633 = t33953 * t5284;
    let t35635 = t31115 * t13299 * t35633;
    let t35643 = t31276 * t8875;
    (t35610, t35616, t35623, t35631, t35633, t35635, t35643)
}
