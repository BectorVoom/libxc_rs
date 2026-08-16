//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 617/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk617(t30: f64, t33: f64, t5572: f64, t1468: f64, t3874: f64, t1344: f64, t2: f64, t580: f64, t605: f64, t1711: f64, t3881: f64, t1348: f64, t1113: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5573 = 0.5848223622634646207e0_f64 * t5572;
    let t5574 = t3874 * t1468;
    let t5577 = t1344 * t2;
    let t5581 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t5574 * t605 + 4.0_f64 / 3.0_f64 * t5577 * t580);
    let t5582 = t3881 * t1711;
    let t5585 = t1348 * t2;
    let t5589 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t5582 * t1113 - 4.0_f64 / 3.0_f64 * t5585 * t580);
    let t5591 = t5581 / 2.0_f64 + t5589 / 2.0_f64;
    (t5573, t5591)
}
