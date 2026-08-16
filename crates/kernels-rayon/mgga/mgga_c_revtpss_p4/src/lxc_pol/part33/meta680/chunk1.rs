//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2215/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2215(t2121: f64, t2247: f64, t5819: f64, t1469: f64, t603: f64, t108737: f64, t108745: f64, t108749: f64, t108759: f64, t108762: f64, t108765: f64, t108816: f64, t2123: f64, t26749: f64, t26755: f64, t29375: f64, t29548: f64, t29554: f64, t6960: f64, t7566: f64, t7576: f64, t7709: f64) -> f64 {
    let t111453 = t2247 * t5819 * t2121;
    let t111457 = t603 * t1469 * t2121;
    let t111468 = 2.0_f64 / 3.0_f64 * t7709 * t29375 + 5.0_f64 / 3.0_f64 * t7566 * t108737 + 5.0_f64 / 6.0_f64 * t26749 * t29548 + 5.0_f64 / 6.0_f64 * t26755 * t29548 + 5.0_f64 / 6.0_f64 * t7566 * t108745 + 5.0_f64 / 6.0_f64 * t7566 * t108749 - 5.0_f64 / 3.0_f64 * t111453 * t6960 + 2.0_f64 / 3.0_f64 * t111457 * t108759 + t108762 * t2123 / 3.0_f64 + t108765 * t2123 / 3.0_f64 + t108816 * t2123 / 3.0_f64 + t29554 * t7576 / 3.0_f64;
    t111468
}
