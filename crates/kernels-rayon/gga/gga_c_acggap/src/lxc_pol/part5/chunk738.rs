//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 738/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk738(t43: f64, t50: f64, t1361: f64, t234: f64, t47: f64, t5445: f64, t5450: f64, t5455: f64, t822: f64, t1699: f64, t2876: f64, t1702: f64, t893: f64, t1369: f64, t238: f64, t52: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t5459 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t5445 * t234 + 16.0_f64 / 9.0_f64 * t1361 * t822 + 4.0_f64 / 9.0_f64 * t5450 * t234 + 4.0_f64 / 3.0_f64 * t47 * t5455);
    let t5460 = t2876 * t1699;
    let t5465 = t893 * t1702;
    let t5468 = -t5455;
    let t5472 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t5460 * t238 - 16.0_f64 / 9.0_f64 * t1369 * t822 + 4.0_f64 / 9.0_f64 * t5465 * t238 + 4.0_f64 / 3.0_f64 * t52 * t5468);
    (t5459, t5460, t5465, t5468, t5472)
}
