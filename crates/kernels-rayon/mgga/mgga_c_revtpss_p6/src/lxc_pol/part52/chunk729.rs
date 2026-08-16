//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 729/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk729(t532: f64, t7535: f64, t1450: f64, t2107: f64, t7315: f64, t118: f64, t1310: f64, t1453: f64, t2014: f64, t2052: f64, t2056: f64, t2089: f64, t2093: f64, t2108: f64, t2322: f64, t4254: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t7235: f64, t7357: f64, t7359: f64, t7367: f64, t7374: f64, t7378: f64, t7474: f64, t7484: f64, t7489: f64) -> (f64, f64, f64, f64) {
    let t7536 = t532 * t7535;
    let t7537 = t7536 * t1450;
    let t7539 = t2107 * t7315;
    let t7541 = -t118 * t7474 - t1310 * t2052 + t1453 * t2093 + 3.0_f64 * t2014 * t7489 + t2014 * t7537 - t2014 * t7539 - 2.0_f64 * t2056 * t2322 - 2.0_f64 * t2056 * t4254 - t2089 * t649 + t2108 * t7235 - t508 * t7357 + t569 * t7484 - 2.0_f64 * t651 * t7367 - 2.0_f64 * t651 * t7374 - 2.0_f64 * t651 * t7378 - 2.0_f64 * t671 * t7359;
    (t7536, t7537, t7539, t7541)
}
