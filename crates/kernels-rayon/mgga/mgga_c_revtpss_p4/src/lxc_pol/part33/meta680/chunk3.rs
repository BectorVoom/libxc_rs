//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2217/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2217(t2247: f64, t30681: f64, t38: f64, t108733: f64, t26749: f64, t26755: f64, t28112: f64, t28116: f64, t28119: f64, t28133: f64, t28141: f64, t29372: f64, t29388: f64, t29544: f64, t30683: f64, t6960: f64, t6963: f64, t7566: f64, t7709: f64, t8144: f64, t8147: f64) -> f64 {
    let t111516 = t2247 * t38 * t30681;
    let t111521 = 5.0_f64 / 3.0_f64 * t26749 * t29544 + 5.0_f64 / 3.0_f64 * t26755 * t29544 + 5.0_f64 / 3.0_f64 * t7566 * t108733 + 2.0_f64 / 3.0_f64 * t28112 * t8147 + 2.0_f64 / 3.0_f64 * t28116 * t8147 + 2.0_f64 / 3.0_f64 * t28119 * t8147 + 2.0_f64 / 3.0_f64 * t7709 * t29372 + 2.0_f64 / 3.0_f64 * t28141 * t8144 + 5.0_f64 / 3.0_f64 * t29388 * t28133 + 2.0_f64 / 3.0_f64 * t28141 * t8147 + 5.0_f64 / 6.0_f64 * t111516 * t6960 + t6963 * t30683 / 3.0_f64;
    t111521
}
