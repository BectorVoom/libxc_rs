//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2253/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2253(t225: f64, t26221: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t5353: f64, t26215: f64, t80650: f64, t12033: f64, t1386: f64, t16439: f64, t22630: f64, t22670: f64, t22913: f64, t26371: f64, t3882: f64, t5215: f64, t5321: f64, t5354: f64, t6963: f64, t7750: f64, t81318: f64, t81328: f64) -> f64 {
    let t91441 = t26221 * t225;
    let t91449 = t22633 * t22635 * t1377 * t5353 * t1307;
    let t91455 = t22633 * t80650 * t26215;
    let t91459 = -6.0_f64 * t5215 * t22630 - t81318 - t12033 * t7750 + 4.0_f64 * t16439 * t6963 - 2.0_f64 * t91441 * t1386 - 2.0_f64 * t22670 * t5354 + 0.3289868133696452873e-1_f64 * t91449 - 0.16449340668482264365e-1_f64 * t81328 + 2.0_f64 * t5321 * t22913 + 0.3289868133696452873e-1_f64 * t91455 + 4.0_f64 * t3882 * t26371;
    t91459
}
