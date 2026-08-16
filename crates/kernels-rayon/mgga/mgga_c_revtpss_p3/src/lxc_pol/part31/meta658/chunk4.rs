//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2226/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2226(t101385: f64, t101391: f64, t28078: f64, t28081: f64, t28086: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t28127: f64, t29538: f64, t6974: f64, t6978: f64, t7706: f64, t7709: f64, t7720: f64) -> f64 {
    let t108854 = 2.0_f64 / 3.0_f64 * t7709 * t28078 + 2.0_f64 / 3.0_f64 * t7709 * t28081 + 2.0_f64 / 3.0_f64 * t28112 * t7720 + 2.0_f64 / 3.0_f64 * t29538 * t6974 + 2.0_f64 / 3.0_f64 * t29538 * t6978 + 5.0_f64 / 3.0_f64 * t101385 * t7706 + 5.0_f64 / 3.0_f64 * t101391 * t7706 + 5.0_f64 / 3.0_f64 * t28127 * t28105 + 5.0_f64 / 3.0_f64 * t28127 * t28109 + 2.0_f64 / 3.0_f64 * t28116 * t7720 + 2.0_f64 / 3.0_f64 * t28119 * t7720 + 2.0_f64 / 3.0_f64 * t7709 * t28086;
    t108854
}
