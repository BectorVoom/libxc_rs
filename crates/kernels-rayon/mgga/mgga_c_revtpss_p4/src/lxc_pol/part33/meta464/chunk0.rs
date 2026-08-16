//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1690/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1690(t5816: f64, t644: f64, t1497: f64, t4241: f64, t5872: f64, t1469: f64, t70: f64, t72: f64, t1927: f64, t4186: f64, t5819: f64, t627: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21674 = t5816 * t644;
    let t21677 = t1497 * t4241;
    let t21682 = t5872 * t644;
    let t21686 = t1469 * t70 * t72;
    let t21687 = t1927 * t4186;
    let t21690 = t5819 * t627;
    (t21674, t21677, t21682, t21686, t21687, t21690)
}
