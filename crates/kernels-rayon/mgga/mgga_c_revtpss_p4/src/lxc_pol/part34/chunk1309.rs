//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1309/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1309(t101252: f64, t101333: f64, t101342: f64, t108880: f64, t108966: f64, t108971: f64, t108979: f64, t108987: f64, t108990: f64, t114246: f64, t114260: f64, t114264: f64, t25157: f64, t28151: f64, t28154: f64, t29562: f64, t92690: f64) -> f64 {
    let t114267 = 30.0_f64 * t101252 * t108880 - 15.0_f64 * t101333 * t29562 - 15.0_f64 * t101342 * t29562 - 10.0_f64 * t108966 * t28151 - 10.0_f64 * t108971 * t28154 - 10.0_f64 * t108979 * t28154 - 5.0_f64 * t108987 * t28154 - 5.0_f64 * t108990 * t28151 - 15.0_f64 * t114246 * t25157 - 15.0_f64 * t114260 * t25157 + 35.0_f64 * t114264 * t92690;
    t114267
}
