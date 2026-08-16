//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 747/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk747(t1861: f64, t997: f64, t1851: f64, t1856: f64, t1894: f64, t336: f64, t372: f64, t4630: f64, t495: f64, t1298: f64, t1501: f64, t1143: f64, t1734: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5579 = t997 * t1861;
    let t5581 = t997 * t1851;
    let t5583 = t997 * t1856;
    let t5586 = t336 * t1894 * t372;
    let t5590 = t336 * t4630 * t495;
    let t5594 = t336 * t1501 * t1298;
    let t5598 = t336 * t1143 * t1734;
    (t5579, t5581, t5583, t5586, t5590, t5594, t5598)
}
