//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 879/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk879(t16729: f64, t787: f64, t3665: f64, t4793: f64, t3681: f64, t16715: f64, t7512: f64, t10188: f64, t10348: f64, t13649: f64, t13651: f64, t13653: f64, t13699: f64, t13701: f64, t16716: f64, t7786: f64, t7787: f64) -> (f64, f64, f64, f64, f64) {
    let t16730 = t787 * t16729;
    let t16732 = t3665 * t4793;
    let t16734 = t3681 * t4793;
    let t16737 = t7512 * t16715;
    let t16741 = 0.69463333333333333335e-1_f64 * t13649 - 0.41678000000000000001e0_f64 * t13651 + 0.20839e0_f64 * t13653 - 0.157790625e0_f64 * t16716 - 0.34731666666666666667e0_f64 * t10348 + 0.6311625e0_f64 * t16730 - 0.52945875e1_f64 * t16732 + 0.94674375e0_f64 * t16734 - 0.68863333333333333332e0_f64 * t10188 - t7786 - t7787 + 0.264729375e1_f64 * t16737 + 0.34431666666666666666e0_f64 * t13699 - 0.103295e1_f64 * t13701;
    (t16730, t16732, t16734, t16737, t16741)
}
