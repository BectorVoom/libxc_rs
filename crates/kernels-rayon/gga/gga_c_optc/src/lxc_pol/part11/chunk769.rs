//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 769/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk769(t1150: f64, t12597: f64, t1119: f64, t141: f64, t3233: f64, t11325: f64, t4456: f64, t4463: f64, t2855: f64, t4298: f64, t2849: f64, t8459: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12598 = t1150 * t12597;
    let t12600 = t1119 * t141;
    let t12601 = t3233 * t12600;
    let t12606 = t4456 * t11325;
    let t12612 = t4463 * t11325;
    let t12617 = t4298 * t2855;
    let t12621 = t8459 * t2849;
    (t12598, t12601, t12606, t12612, t12617, t12621)
}
