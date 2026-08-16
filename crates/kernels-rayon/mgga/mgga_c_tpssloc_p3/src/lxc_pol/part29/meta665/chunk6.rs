//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2216/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2216(t562: f64, t80854: f64, t16226: f64, t90914: f64, t22685: f64, t26395: f64, t3734: f64, t6637: f64, t81080: f64, t16125: f64, t1992: f64, t6976: f64) -> (f64, f64, f64, f64, f64) {
    let t90915 = t80854 * t562;
    let t90917 = t90914 * t90915 * t16226;
    let t90921 = t22685 * t6637 * t26395 * t3734;
    let t90925 = 0.10417915756705434098e0_f64 * t81080;
    let t90929 = t1992 * t6976 * t16125;
    (t90915, t90917, t90921, t90925, t90929)
}
