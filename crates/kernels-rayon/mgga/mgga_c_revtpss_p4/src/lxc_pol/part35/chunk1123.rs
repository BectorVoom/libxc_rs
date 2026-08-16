//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1123/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1123(t45972: f64, t7342: f64, t10309: f64, t26178: f64, t94973: f64, t25373: f64, t26550: f64, t25386: f64, t26518: f64, t9285: f64, t25299: f64, t2061: f64, t22: f64, t25402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95397 = 308.0_f64 / 27.0_f64 * t94973;
    let t95536 = t25373 * t26550;
    let t95537 = t25386 * t95536;
    let t95540 = t26518 * t9285;
    let t95542 = 0.68540937416128198417e-2_f64 * t25299 * t95540;
    let t95546 = t25402 * t2061 * t22;
    (t95316, t95319, t95397, t95536, t95537, t95540, t95542, t95546)
}
