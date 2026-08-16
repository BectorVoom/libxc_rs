//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 851/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk851(t157: f64, t1734: f64, t406: f64, t1487: f64, t524: f64, t1795: f64, t1410: f64, t1748: f64, t1854: f64, t322: f64, t7158: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25941 = t1734 * t406 * t157;
    let t26108 = t1487 * t524 * t157;
    let t26214 = t1795 * t406 * t157;
    let t26459 = t1748 * t1410;
    let t26554 = t1854 * t322;
    let t26757 = t7158 * t406;
    let t26956 = t1854 * t372;
    (t25941, t26108, t26214, t26459, t26554, t26757, t26956)
}
