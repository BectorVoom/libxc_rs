//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1373/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1373(t111453: f64, t111516: f64, t111537: f64, t111675: f64, t114288: f64, t28154: f64, t29388: f64, t29412: f64, t29538: f64, t29544: f64, t30683: f64, t30686: f64, t30689: f64, t7566: f64, t7706: f64, t7709: f64, t8144: f64, t8147: f64) -> f64 {
    let t116844 = -5.0_f64 * t28154 * t111675 + 5.0_f64 * t111537 * t7706 + 2.0_f64 * t29538 * t8144 + 5.0_f64 * t29388 * t29544 + 2.0_f64 * t29538 * t8147 + 5.0_f64 / 2.0_f64 * t111516 * t7706 + t7709 * t30683 + 5.0_f64 * t29412 * t29544 + 2.0_f64 * t7709 * t30686 + 5.0_f64 / 2.0_f64 * t7566 * t114288 + t7709 * t30689 - 5.0_f64 * t111453 * t7706;
    t116844
}
