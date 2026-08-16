//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1151/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1151(t2030: f64, t507: f64, t8816: f64, t1488: f64, t2060: f64, t2317: f64, t13287: f64, t31057: f64, t38857: f64, t1181: f64, t5651: f64, t599: f64, t8463: f64) -> (f64, f64, f64, f64) {
    let t39907 = t2030 * t507 * t8816;
    let t39910 = t2060 * t1488 * t2317;
    let t39914 = t31057 * t13287 * t38857;
    let t39919 = t8463 * t1181 * t599 * t5651;
    (t39907, t39910, t39914, t39919)
}
