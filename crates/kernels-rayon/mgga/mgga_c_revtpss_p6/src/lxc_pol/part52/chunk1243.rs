//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1243/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1243(t119457: f64, t128409: f64, t640: f64, t121617: f64, t121625: f64, t121630: f64, t121646: f64, t121647: f64, t121660: f64, t125238: f64, t125265: f64, t125274: f64, t128368: f64, t128371: f64, t128374: f64, t128377: f64, t128382: f64, t128385: f64, t128394: f64, t128399: f64, t128403: f64, t128411: f64, t32151: f64, t32581: f64, t32590: f64, t32593: f64, t33621: f64, t34169: f64, t34177: f64, t45972: f64, t60221: f64, t8619: f64, t8620: f64, t8623: f64) -> f64 {
    let t128415 = t119457 * t128409 * t640;
    let t128422 = -5.0_f64 / 3.0_f64 * t121660 * t128368 - 5.0_f64 / 3.0_f64 * t121660 * t128371 + 5.0_f64 / 9.0_f64 * t32590 * t128374 + 5.0_f64 / 9.0_f64 * t32590 * t128377 - 5.0_f64 / 72.0_f64 * t8620 * t125238 + 5.0_f64 / 27.0_f64 * t128382 + 5.0_f64 / 27.0_f64 * t128385 - 5.0_f64 / 72.0_f64 * t60221 * t8619 * t8623 - 5.0_f64 / 72.0_f64 * t34169 * t32151 - 5.0_f64 / 72.0_f64 * t32581 * t33621 + 5.0_f64 / 18.0_f64 * t128394 * t32593 + 5.0_f64 / 27.0_f64 * t121617 - 20.0_f64 / 27.0_f64 * t121630 - 20.0_f64 / 27.0_f64 * t128399 + 5.0_f64 / 6.0_f64 * t121647 * t128403 + 5.0_f64 / 18.0_f64 * t32590 * t125274 - 35.0_f64 / 12.0_f64 * t45972 * t121646 * t128411 + 5.0_f64 / 6.0_f64 * t121647 * t128415 + 5.0_f64 / 18.0_f64 * t121625 * t34177 + 5.0_f64 / 18.0_f64 * t32590 * t125265;
    t128422
}
