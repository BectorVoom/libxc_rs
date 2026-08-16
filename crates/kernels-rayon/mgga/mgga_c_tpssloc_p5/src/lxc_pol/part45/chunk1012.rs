//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1012/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1012(t31618: f64, t3719: f64, t6637: f64, t6888: f64, t22892: f64, t22893: f64, t31619: f64, t22685: f64, t3734: f64, t31628: f64, t6914: f64, t114056: f64, t115387: f64, t115391: f64, t115395: f64, t115397: f64, t115402: f64, t3773: f64, t8634: f64) -> f64 {
    let t115406 = t6888 * t6637 * t31618 * t3719;
    let t115409 = t22892 * t22893 * t31619;
    let t115413 = t22685 * t6637 * t31618 * t3734;
    let t115415 = t6914 * t31628;
    let t115417 = -0.82246703342411321825e-2_f64 * t115387 - t115391 + t3773 * t8634 + t114056 + 0.16449340668482264365e-1_f64 * t115395 + 0.76763589786250567036e-1_f64 * t115397 - 0.3289868133696452873e-1_f64 * t115402 - 0.16449340668482264365e-1_f64 * t115406 + 0.16449340668482264365e-1_f64 * t115409 + 0.49348022005446793095e-1_f64 * t115413 + 0.38381794893125283518e-1_f64 * t115415;
    t115417
}
