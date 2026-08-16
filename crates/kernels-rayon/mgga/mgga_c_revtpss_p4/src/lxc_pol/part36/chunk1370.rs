//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1370/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1370(t114322: f64, t114343: f64, t114349: f64, t1923: f64, t2122: f64, t2123: f64, t29513: f64, t29532: f64, t29551: f64, t30683: f64, t30686: f64, t30689: f64, t7702: f64, t8143: f64, t8144: f64, t8147: f64) -> f64 {
    let t116759 = -t1923 * t8143 * t29532 / 2.0_f64 - t1923 * t2122 * t114343 / 6.0_f64 + t29551 * t8144 + t29551 * t8147 + t114322 * t2123 - t114349 * t2123 / 6.0_f64 - t29513 * t8144 / 2.0_f64 - t29513 * t8147 / 2.0_f64 - t7702 * t30683 / 2.0_f64 - t7702 * t30686 - t7702 * t30689 / 2.0_f64;
    t116759
}
