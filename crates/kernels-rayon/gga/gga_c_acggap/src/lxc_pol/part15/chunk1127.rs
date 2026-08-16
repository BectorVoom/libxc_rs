//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1127/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1127(t1849: f64, t322: f64, t1165: f64, t7351: f64, t7493: f64, t5608: f64, t7561: f64, t1844: f64, t604: f64, t7346: f64, t1181: f64, t2068: f64, t39164: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39491 = t1849 * t322;
    let t39494 = t7493 * t1165 * t7351 * t39491;
    let t39497 = t7561 * t5608;
    let t39499 = t1844 * t322;
    let t39502 = t7346 * t1165 * t604 * t39499;
    let t39506 = t2068 * t1181 * t604 * t39164;
    (t39491, t39494, t39497, t39499, t39502, t39506)
}
