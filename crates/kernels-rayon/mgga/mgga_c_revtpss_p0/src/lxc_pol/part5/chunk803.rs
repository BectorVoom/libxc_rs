//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 803/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk803(t30: f64, t189: f64, t5566: f64, t512: f64, t1856: f64, t749: f64, t177: f64, t762: f64, t1468: f64, t3874: f64, t1344: f64, t2: f64, t580: f64, t605: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t5567 = t5566 * t189;
    let t5568 = t512 * t5567;
    let t5569 = t1856 * t749;
    let t5570 = t512 * t5569;
    let t5571 = t1856 * t177;
    let t5572 = t5571 * t762;
    let t5573 = 0.5848223622634646207e0_f64 * t5572;
    let t5574 = t3874 * t1468;
    let t5577 = t1344 * t2;
    let t5581 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t5574 * t605 + 4.0_f64 / 3.0_f64 * t5577 * t580);
    (t5567, t5568, t5569, t5570, t5571, t5572, t5573, t5574, t5581)
}
