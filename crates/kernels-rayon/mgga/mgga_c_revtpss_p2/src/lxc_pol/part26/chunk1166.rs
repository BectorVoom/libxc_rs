//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1166/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1166(t198: f64, t206: f64, t7427: f64, t2411: f64, t26580: f64, t25373: f64, t26550: f64, t25386: f64, t92840: f64, t26518: f64, t9285: f64, t25299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95511 = t198 * t206 * t7427;
    let t95527 = t26580 * t2411;
    let t95536 = t25373 * t26550;
    let t95537 = t25386 * t95536;
    let t95538 = t95537 * t92840;
    let t95540 = t26518 * t9285;
    let t95542 = 0.68540937416128198417e-2_f64 * t25299 * t95540;
    (t95511, t95527, t95536, t95538, t95540, t95542)
}
