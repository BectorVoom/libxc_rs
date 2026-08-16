//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 662/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk662(t2101: f64, t597: f64, t24: f64, t7368: f64, t603: f64, t157: f64, t161: f64, t7943: f64, t89: f64, t2252: f64, t342: f64, t657: f64) -> (f64, f64, f64, f64, f64) {
    let t9419 = t2101 * t597;
    let t9432 = t24 * t7368;
    let t9437 = t603 * t603;
    let t9438 = 1.0_f64 / t9437;
    let t9439 = t157 * t9438;
    let t9457 = 28.0_f64 / 81.0_f64 * t89 * t7943 * t161;
    let t9482 = t342 * t2252 * t657 / 18.0_f64;
    (t9419, t9432, t9439, t9457, t9482)
}
