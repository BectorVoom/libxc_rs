//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 749/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk749(t571: f64, t7852: f64, t336: f64, t3565: f64, t570: f64, t1072: f64, t154: f64, t7322: f64, t1: f64, t145: f64, t203: f64, t2020: f64, t2025: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7853 = t7852 * t571;
    let t7854 = 35.0_f64 / 432.0_f64 * t7853;
    let t7855 = t336 * t3565;
    let t7856 = t570 * t7855;
    let t7858 = t154 * t1072;
    let t7859 = t7322 * t7858;
    let t7861 = t145 * t1 * t203;
    let t7862 = t7859 * t7861;
    let t7863 = t7862 / 384.0_f64;
    let t7864 = t2020 * t2025;
    (t7854, t7855, t7856, t7858, t7859, t7861, t7863, t7864)
}
