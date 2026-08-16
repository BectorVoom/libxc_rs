//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2216/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2216(t104181: f64, t104185: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t29364: f64, t29367: f64, t29412: f64, t29538: f64, t29554: f64, t7576: f64, t7579: f64, t7706: f64, t7709: f64, t8144: f64) -> f64 {
    let t111493 = t29554 * t7579 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t28112 * t8144 + 2.0_f64 / 3.0_f64 * t28116 * t8144 + 2.0_f64 / 3.0_f64 * t28119 * t8144 + 2.0_f64 / 3.0_f64 * t7709 * t29364 + 2.0_f64 / 3.0_f64 * t7709 * t29367 + 2.0_f64 / 3.0_f64 * t29538 * t7576 + 2.0_f64 / 3.0_f64 * t29538 * t7579 + 5.0_f64 / 3.0_f64 * t104181 * t7706 + 5.0_f64 / 3.0_f64 * t104185 * t7706 + 5.0_f64 / 3.0_f64 * t29412 * t28105 + 5.0_f64 / 3.0_f64 * t29412 * t28109;
    t111493
}
