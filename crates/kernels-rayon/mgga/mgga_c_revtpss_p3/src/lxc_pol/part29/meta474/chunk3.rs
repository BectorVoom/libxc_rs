//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1747/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1747(t10416: f64, t118: f64, t13435: f64, t1453: f64, t2014: f64, t2052: f64, t2056: f64, t2108: f64, t2322: f64, t2331: f64, t25082: f64, t25188: f64, t26380: f64, t26383: f64, t26392: f64, t26396: f64, t26399: f64, t26406: f64, t26412: f64, t26415: f64, t26674: f64, t26676: f64, t26679: f64, t26699: f64, t3813: f64, t508: f64, t569: f64, t651: f64, t671: f64, t7235: f64, t7359: f64, t7367: f64, t7484: f64, t7537: f64) -> f64 {
    let t26702 = -2.0_f64 * t2014 * t26380 + 3.0_f64 * t2014 * t26383 - 2.0_f64 * t10416 * t2056 - 4.0_f64 * t13435 * t2056 - 4.0_f64 * t2322 * t7367 - t2014 * t26392 - 4.0_f64 * t7359 * t2331 - 4.0_f64 * t651 * t26396 - 4.0_f64 * t26399 * t671 + 2.0_f64 * t7484 * t1453 + t25188 * t2108 - 6.0_f64 * t25082 * t26406 + 2.0_f64 * t7235 * t7537 + 6.0_f64 * t2014 * t26412 - 2.0_f64 * t651 * t26415 - t118 * t26674 - 2.0_f64 * t26676 * t508 + 2.0_f64 * t2014 * t26679 + t26699 * t569 - t2052 * t3813;
    t26702
}
