//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 882/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk882(t435: f64, t864: f64, t1096: f64, t1165: f64, t12991: f64, t3809: f64, t388: f64, t1084: f64, t1181: f64, t12936: f64, t3646: f64, t396: f64) -> (f64, f64, f64, f64, f64) {
    let t12992 = t435 * t864;
    let t12995 = t12991 * t1165 * t12992 * t1096;
    let t12999 = t12991 * t1165 * t388 * t3809;
    let t13031 = t12936 * t1181 * t12992 * t1084;
    let t13039 = t3646 * t396;
    (t12992, t12995, t12999, t13031, t13039)
}
