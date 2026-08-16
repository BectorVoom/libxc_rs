//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1260/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1260(t104527: f64, t1276: f64, t2148: f64, t1234: f64, t29082: f64, t17416: f64, t7624: f64, t17376: f64, t26843: f64, t26848: f64, t17400: f64, t26866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104529 = t2148 * t104527 * t1276;
    let t104636 = t1234 * t29082;
    let t104658 = t7624 * t17416;
    let t104682 = t17376 * t26843;
    let t104685 = t17376 * t26848;
    let t104703 = t17400 * t26866;
    (t104529, t104636, t104658, t104682, t104685, t104703)
}
