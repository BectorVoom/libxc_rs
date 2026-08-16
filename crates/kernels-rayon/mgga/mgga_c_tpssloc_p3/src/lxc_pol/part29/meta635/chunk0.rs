//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2087/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2087(t22960: f64, t59580: f64, t1408: f64, t2745: f64, t25365: f64, t81547: f64, t1530: f64, t2553: f64, t12971: f64, t25: f64, t2379: f64, t4255: f64, t606: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t86803 = t22960 * t59580;
    let t86806 = t1408 * t2745;
    let t86810 = t81547 * t25365;
    let t86815 = t1530 * t2553;
    let t86816 = t22960 * t86815;
    let t86821 = t25 * t12971;
    let t86825 = t1408 * t2379;
    let t86830 = t870 * t606 * t4255;
    (t86803, t86806, t86810, t86815, t86816, t86821, t86825, t86830)
}
