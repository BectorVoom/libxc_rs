//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2213/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2213(t101129: f64, t101132: f64, t101190: f64, t101193: f64, t101350: f64, t2123: f64, t25102: f64, t25120: f64, t28112: f64, t28116: f64, t29372: f64, t6963: f64, t7566: f64, t7576: f64, t7579: f64, t8147: f64) -> f64 {
    let t104274 = 2.0_f64 / 3.0_f64 * t101190 * t2123 + 2.0_f64 / 3.0_f64 * t101193 * t2123 + 2.0_f64 / 3.0_f64 * t28112 * t7576 + 2.0_f64 / 3.0_f64 * t28112 * t7579 + t101129 * t2123 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t101132 * t2123 + 2.0_f64 / 3.0_f64 * t28116 * t7576 + 2.0_f64 / 3.0_f64 * t28116 * t7579 + 5.0_f64 / 6.0_f64 * t7566 * t101350 + 2.0_f64 / 3.0_f64 * t25102 * t8147 + t25120 * t8147 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6963 * t29372;
    t104274
}
