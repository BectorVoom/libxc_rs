//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1183/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1183(t30: f64, t33: f64, t5571: f64, t762: f64, t1468: f64, t3874: f64, t1344: f64, t2: f64, t580: f64, t605: f64, t1711: f64, t3881: f64, t1348: f64, t1113: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5572 = t5571 * t762;
    let t5573 = 0.5848223622634646207e0_f64 * t5572;
    let t5574 = t3874 * t1468;
    let t5577 = t1344 * t2;
    let t5581 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t5574 * t605 + 4.0_f64 / 3.0_f64 * t5577 * t580);
    let t5582 = t3881 * t1711;
    let t5585 = t1348 * t2;
    let t5589 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t5582 * t1113 - 4.0_f64 / 3.0_f64 * t5585 * t580);
    (t5572, t5573, t5574, t5581, t5582, t5589)
}
