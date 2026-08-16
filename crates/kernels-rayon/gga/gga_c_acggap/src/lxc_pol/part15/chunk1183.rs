//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1183/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1183(t1165: f64, t5537: f64, t7564: f64, t8600: f64, t30219: f64, t9670: f64, t7839: f64, t9674: f64, t8480: f64, t8613: f64, t1181: f64, t604: f64, t6079: f64, t7426: f64) -> (f64, f64, f64, f64, f64) {
    let t40485 = t7564 * t1165 * t8600 * t5537;
    let t40487 = t30219 * t9670;
    let t40490 = t7839 * t9674;
    let t40493 = t7564 * t8480 * t8613;
    let t40497 = t7426 * t1181 * t604 * t6079;
    (t40485, t40487, t40490, t40493, t40497)
}
