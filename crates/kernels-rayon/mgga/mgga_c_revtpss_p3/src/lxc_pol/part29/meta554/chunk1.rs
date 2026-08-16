//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1895/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1895(t26249: f64, t9664: f64, t25895: f64, t96264: f64, t25899: f64, t96431: f64, t1445: f64, t26354: f64, t689: f64, t1426: f64, t7507: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t96564 = 0.46263278077393568556e-2_f64 * t26249 * t9664;
    let t96565 = t25895 * t96264;
    let t96567 = t25899 * t96431;
    let t96570 = t689 * t26354 * t1445;
    let t96576 = t786 * t7507 * t1426;
    (t96564, t96565, t96567, t96570, t96576)
}
