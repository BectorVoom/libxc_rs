//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 587/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk587(t50: f64, t5455: f64, t1369: f64, t238: f64, t52: f64, t5460: f64, t5465: f64, t822: f64, t5459: f64, t59: f64, t85: f64, t4030: f64, t2635: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t5468 = -t5455;
    let t5472 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t5460 * t238 - 16.0_f64 / 9.0_f64 * t1369 * t822 + 4.0_f64 / 9.0_f64 * t5465 * t238 + 4.0_f64 / 3.0_f64 * t52 * t5468);
    let t5474 = (t5459 + t5472) * t59;
    let t5475 = t5474 * t85;
    let t5476 = 0.19751673498613801407e-1_f64 * t5475;
    let t5477 = 0.48830526149350786811e-3_f64 * t4030;
    let t5478 = 12.0_f64 * t2635;
    (t5468, t5474, t5476, t5477, t5478)
}
