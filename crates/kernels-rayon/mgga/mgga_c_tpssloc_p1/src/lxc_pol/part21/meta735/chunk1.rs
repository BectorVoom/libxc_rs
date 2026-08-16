//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2595/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2595(t3609: f64, t52434: f64, t1011: f64, t1212: f64, t52446: f64, t11539: f64, t1174: f64, t14736: f64, t1227: f64, t13969: f64, t15544: f64, t15655: f64) -> (f64, f64, f64, f64, f64) {
    let t52485 = t52434 * t3609;
    let t52568 = t52446 * t1011 * t1212;
    let t52575 = t1174 * t11539 * t14736;
    let t52580 = t1227 * t13969 * t15544;
    let t52583 = t1227 * t13969 * t15655;
    (t52485, t52568, t52575, t52580, t52583)
}
