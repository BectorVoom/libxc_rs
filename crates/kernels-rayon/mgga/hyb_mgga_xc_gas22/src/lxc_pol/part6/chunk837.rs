//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 837/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk837(t2024: f64, t2029: f64, t6479: f64, t2004: f64, t680: f64, t2011: f64, t677: f64, t2187: f64, t783: f64, t222: f64, t226: f64, t6007: f64) -> (f64, f64, f64, f64, f64) {
    let t6481 = t2024 * t6479 * t2029;
    let t6483 = t2004 * t680;
    let t6485 = t677 * t2011;
    let t6497 = t783 * t2187;
    let t6527 = t222 * t6007 * t226;
    (t6481, t6483, t6485, t6497, t6527)
}
