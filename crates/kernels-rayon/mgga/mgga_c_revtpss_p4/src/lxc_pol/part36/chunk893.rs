//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 893/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk893(t1222: f64, t17628: f64, t372: f64, t5277: f64, t1778: f64, t3682: f64, t1770: f64, t3766: f64, t3754: f64, t5219: f64, t1811: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17629 = t1222 * t17628;
    let t17661 = t372 * t5277;
    let t17792 = t1778 * t3682;
    let t17934 = t1770 * t3766;
    let t17958 = t5219 * t3754;
    let t17995 = t3566 * t1811;
    (t17629, t17661, t17792, t17934, t17958, t17995)
}
