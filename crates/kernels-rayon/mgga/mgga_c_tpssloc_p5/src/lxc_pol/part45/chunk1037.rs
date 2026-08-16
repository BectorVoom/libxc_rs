//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1037/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1037(t2018: f64, t22574: f64, t24432: f64, t3719: f64, t115723: f64, t2039: f64, t31537: f64, t7056: f64, t22479: f64, t88: f64, t31717: f64, t23917: f64, t8601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115781 = 3.0_f64 * t22574 * t24432 * t2018 * t3719;
    let t115783 = 4.0_f64 * t115723 * t2039;
    let t115785 = 4.0_f64 * t31537 * t7056;
    let t115786 = t88 * t22479;
    let t115788 = 2.0_f64 * t115786 * t2039;
    let t115790 = 4.0_f64 * t31717 * t7056;
    let t115792 = 2.0_f64 * t8601 * t23917;
    (t115781, t115783, t115785, t115788, t115790, t115792)
}
