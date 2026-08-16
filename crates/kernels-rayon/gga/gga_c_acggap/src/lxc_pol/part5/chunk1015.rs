//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1015/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1015(t13292: f64, t3073: f64, t1106: f64, t13364: f64, t406: f64, t8790: f64, t1530: f64, t13285: f64, t13298: f64, t176: f64, t5284: f64, t8401: f64) -> (f64, f64, f64, f64, f64) {
    let t17173 = t3073 * t13292;
    let t17177 = t17173 * t13364 * t8790 * t1106 * t406;
    let t17179 = t1530 * t13292;
    let t17185 = t1530 * t13285;
    let t17198 = t13298 * t176 * t8401 * t5284;
    (t17173, t17177, t17179, t17185, t17198)
}
