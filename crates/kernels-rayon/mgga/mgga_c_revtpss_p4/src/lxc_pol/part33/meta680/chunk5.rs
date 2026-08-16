//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2219/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2219(t10309: f64, t104317: f64, t108807: f64, t108810: f64, t108813: f64, t1470: f64, t2121: f64, t2123: f64, t28093: f64, t28105: f64, t28109: f64, t28147: f64, t28154: f64, t29388: f64, t29513: f64, t29551: f64, t7576: f64, t7579: f64, t8144: f64) -> f64 {
    let t111577 = 5.0_f64 / 3.0_f64 * t29388 * t28105 + 5.0_f64 / 3.0_f64 * t29388 * t28109 + 2.0_f64 / 3.0_f64 * t108807 * t2123 + 2.0_f64 / 3.0_f64 * t108810 * t2123 + 2.0_f64 / 3.0_f64 * t108813 * t2123 - 10.0_f64 / 3.0_f64 * t28154 * t104317 + 20.0_f64 * t10309 * t1470 * t2121 * t28147 - t29513 * t7576 / 6.0_f64 - t29513 * t7579 / 6.0_f64 - t28093 * t8144 / 3.0_f64 + t29551 * t7576 / 3.0_f64 + t29551 * t7579 / 3.0_f64;
    t111577
}
