//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 971/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk971<F: Float>(t30: F, t189: F, t5566: F, t512: F, t1856: F, t749: F, t177: F, t762: F, t1468: F, t3874: F, t1344: F, t2: F, t580: F, t605: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t5567 = t5566 * t189;
    let t5568 = t512 * t5567;
    let t5569 = t1856 * t749;
    let t5570 = t512 * t5569;
    let t5571 = t1856 * t177;
    let t5572 = t5571 * t762;
    let t5573 = F::cast_from(0.5848223622634646207e0_f64) * t5572;
    let t5574 = t3874 * t1468;
    let t5577 = t1344 * t2;
    let t5581 = piecewise3::<F>(t31, F::new(0.0), -F::new(2.0) / F::new(9.0) * t5574 * t605 + F::new(4.0) / F::new(3.0) * t5577 * t580);
    (t5567, t5568, t5569, t5570, t5571, t5572, t5573, t5574, t5581)
}
