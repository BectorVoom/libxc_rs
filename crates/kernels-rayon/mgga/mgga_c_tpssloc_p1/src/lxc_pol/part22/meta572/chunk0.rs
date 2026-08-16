//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2080/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2080(t43603: f64, t68: f64, t3215: f64, t3399: f64, t3402: f64, t3639: f64, t11545: f64, t241: f64, t3241: f64, t242: f64, t281: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43604 = t68 * t43603;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0_f64 / t43636;
    let t43688 = t3399 * t3399;
    let t43689 = 1.0_f64 / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = 1.0_f64 / t43691;
    let t43705 = t3639 * t3639;
    let t43706 = 1.0_f64 / t43705;
    let t43761 = t241 * t11545;
    let t43762 = t3241 * t3241;
    let t43763 = 1.0_f64 / t43762;
    let t43776 = t281 * t242 * t415;
    (t43604, t43637, t43689, t43692, t43706, t43761, t43763, t43776)
}
