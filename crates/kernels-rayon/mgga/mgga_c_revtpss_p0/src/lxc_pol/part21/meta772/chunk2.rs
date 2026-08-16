//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2744/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2744(t10726: f64, t10943: f64, t2661: f64, t4352: f64, t14547: f64, t40693: f64, t14917: f64, t1558: f64, t2475: f64, t2662: f64, t14724: f64, t9775: f64) -> (f64, f64, f64, f64) {
    let t50493 = t2661 * t10726 * t4352 * t10943;
    let t50497 = t2661 * t40693 * t4352 * t14547;
    let t50502 = t2661 * t2662 * t2475 * t1558 * t14917;
    let t50504 = t9775 * t14724;
    (t50493, t50497, t50502, t50504)
}
