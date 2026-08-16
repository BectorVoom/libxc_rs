//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1148/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1148(t15386: f64, t31443: f64, t39858: f64, t1745: f64, t2012: f64, t31346: f64, t5903: f64, t35466: f64, t6339: f64, t6086: f64, t7822: f64, t1181: f64, t26554: f64, t7351: f64, t7426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39860 = t31443 * t15386 * t39858;
    let t39862 = t2012 * t1745;
    let t39867 = t31346 * t5903;
    let t39869 = t35466 * t6339;
    let t39871 = t7822 * t6086;
    let t39876 = t7426 * t1181 * t7351 * t26554;
    (t39860, t39862, t39867, t39869, t39871, t39876)
}
