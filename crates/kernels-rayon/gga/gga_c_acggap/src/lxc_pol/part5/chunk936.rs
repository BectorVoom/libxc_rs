//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 936/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk936(t12730: f64, t180: f64, t3037: f64, t407: f64, t1160: f64, t3065: f64, t955: f64, t3073: f64, t945: f64, t1237: f64, t13259: f64, t3066: f64, t3077: f64) -> (f64, f64, f64, f64, f64) {
    let t14525 = t12730 * t180 * t3037 * t407;
    let t14528 = t1160 * t3065 * t955;
    let t14534 = t3073 * t3065 * t945;
    let t14539 = t13259 * t1237;
    let t14547 = t3077 * t3066;
    (t14525, t14528, t14534, t14539, t14547)
}
