//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 668/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk668(t289: f64, t9639: f64, t2448: f64, t504: f64, t2479: f64, t275: f64, t2231: f64, t534: f64, t72: f64, t530: f64, t8188: f64, t2474: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9640 = t289 * t9639;
    let t9642 = t504 * t2448;
    let t9650 = t275 * t2479;
    let t9658 = t534 * t2231;
    let t9659 = t72 * t9658;
    let t9675 = t530 * t8188;
    let t9677 = t302 * t2474;
    (t9640, t9642, t9650, t9658, t9659, t9675, t9677)
}
