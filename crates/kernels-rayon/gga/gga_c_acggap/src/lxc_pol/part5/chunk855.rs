//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 855/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk855(t11995: f64, t12058: f64, t12091: f64, t12144: f64, t40: f64, t60: f64, t11870: f64, t272: f64, t2773: f64, t286: f64, t699: f64, t712: f64) -> (f64, f64, f64) {
    let t12148 = t40 * t60 * (t11995 + t12058 + t12091 + t12144);
    let t12156 = 0.14035736694323150897e2_f64 * t286 * t2773 * t11870 * t272;
    let t12157 = t712 * t699;
    (t12148, t12156, t12157)
}
