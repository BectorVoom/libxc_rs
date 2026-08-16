//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1355/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1355(t2496: f64, t5571: f64, t9597: f64, t123: f64, t1856: f64, t2630: f64, t1857: f64, t3860: f64, t5566: f64, t749: f64, t512: f64, t9856: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13652 = t5571 * t2496;
    let t13664 = 12.0_f64 * t9597;
    let t13665 = t1856 * t123;
    let t13666 = t13665 * t2630;
    let t13668 = t3860 * t1857;
    let t13680 = t5566 * t749;
    let t13682 = 2.0_f64 * t512 * t13680;
    let t13683 = 48.0_f64 * t9856;
    (t13652, t13664, t13666, t13668, t13682, t13683)
}
