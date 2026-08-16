//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 405/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk405(t143: f64, t2379: f64, t126: f64, t684: f64, t15: f64, t60: f64, t762: f64, t647: f64, t130: f64, t20: f64, t21: f64, t736: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2542 = t2379 * t143;
    let t2545 = t684 * t126;
    let t2546 = t2545 * t15;
    let t2551 = t60 * t762;
    let t2552 = t2551 * t647;
    let t2553 = t130 * t20;
    let t2555 = t2553 * t21 * t736;
    (t2542, t2545, t2546, t2551, t2552, t2553, t2555)
}
