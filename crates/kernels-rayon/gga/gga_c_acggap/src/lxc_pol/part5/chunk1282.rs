//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1282/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1282(t1439: f64, t360: f64, t1165: f64, t14368: f64, t1884: f64, t4210: f64, t1416: f64, t372: f64, t1345: f64, t322: f64, t13298: f64, t13299: f64, t525: f64) -> (f64, f64, f64, f64) {
    let t23718 = t1439 * t360;
    let t23725 = t14368 * t1165 * t1884 * t4210;
    let t23736 = t1416 * t372;
    let t23745 = t1345 * t322;
    let t23748 = t13298 * t13299 * t525 * t23745;
    (t23718, t23725, t23736, t23748)
}
