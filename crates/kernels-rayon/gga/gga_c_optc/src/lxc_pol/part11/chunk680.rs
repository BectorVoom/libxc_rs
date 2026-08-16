//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 680/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk680(t1789: f64, t508: f64, t1793: f64, t209: f64, t110: f64, t514: f64, t535: f64, t1756: f64, t1759: f64, t580: f64, t1864: f64, t565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6481 = t508 * t1789;
    let t6484 = 0.85917146441092277512e0_f64 * t209 * t6481 * t1793;
    let t6485 = t110 * t514;
    let t6488 = 0.71233333333333333334e-1_f64 * t209 * t6485 * t535;
    let t6492 = 0.10685e0_f64 * t209 * t508 * t1756 * t1759;
    let t6493 = t110 * t580;
    let t6500 = t508 * t1864;
    let t6504 = t110 * t565;
    (t6481, t6484, t6485, t6488, t6492, t6493, t6500, t6504)
}
