//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1432/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1432(t30: f64, t5566: f64, t749: f64, t512: f64, t9856: f64, t1468: f64, t9605: f64, t2: f64, t3874: f64, t1344: f64, t13554: f64, t22: f64, t2257: f64, t3834: f64, t5574: f64, t5577: f64, t580: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t13680 = t5566 * t749;
    let t13682 = 2.0_f64 * t512 * t13680;
    let t13683 = 48.0_f64 * t9856;
    let t13687 = t9605 * t1468;
    let t13690 = t3874 * t2;
    let t13700 = piecewise3(t31, 0.0_f64, 8.0_f64 / 27.0_f64 * t13687 * t3834 - 8.0_f64 / 9.0_f64 * t13690 * t13554 - 2.0_f64 / 9.0_f64 * t5574 * t2257 + 4.0_f64 / 3.0_f64 * t1344 * t580 - 4.0_f64 * t5577 * t22);
    (t13682, t13683, t13700)
}
