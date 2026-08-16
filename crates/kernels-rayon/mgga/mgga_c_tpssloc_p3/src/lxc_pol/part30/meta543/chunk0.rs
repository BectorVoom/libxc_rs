//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1893/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1893(t12524: f64, t7769: f64, t20173: f64, t1458: f64, t6534: f64, t3941: f64, t1873: f64, t4072: f64, t3938: f64, t7467: f64, t671: f64, t1401: f64, t26135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26539 = 27.0_f64 * t12524 * t7769;
    let t26541 = 27.0_f64 * t20173 * t7769;
    let t26542 = t6534 * t1458;
    let t26544 = 27.0_f64 * t3941 * t26542;
    let t26545 = t1873 * t4072;
    let t26547 = 27.0_f64 * t3941 * t26545;
    let t26549 = 0.135e2_f64 * t3938 * t7467;
    let t26550 = t7467 * t671;
    let t26552 = 27.0_f64 * t3941 * t26550;
    let t26554 = 0.135e2_f64 * t1401 * t26135;
    (t26539, t26541, t26542, t26544, t26545, t26547, t26549, t26550, t26552, t26554)
}
