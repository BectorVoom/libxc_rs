//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 707/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk707(t884: f64, t9624: f64, t5888: f64, t8041: f64, t1356: f64, t9531: f64, t2474: f64, t290: f64, t289: f64, t2448: f64, t504: f64, t2479: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9625 = t884 * t9624;
    let t9627 = t8041 * t5888;
    let t9628 = t1356 * t9627;
    let t9637 = t1356 * t9531;
    let t9639 = t290 * t2474;
    let t9640 = t289 * t9639;
    let t9642 = t504 * t2448;
    let t9650 = t275 * t2479;
    (t9625, t9627, t9628, t9637, t9639, t9640, t9642, t9650)
}
