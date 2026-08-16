//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 813/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk813(t127: f64, t1664: f64, t3280: f64, t3282: f64, t3284: f64, t3288: f64, t3290: f64, t411: f64, t5564: f64, t5565: f64, t5570: f64, t5571: f64, t5577: f64, t5578: f64, t5614: f64) -> f64 {
    let t5616 = t5564 + t3280 - t3282 - t3284 - t3288 - t3290 - 1.46904_f64 * t127 * t5565 - t5570 - 29.3808_f64 * t127 * t5571 * t1664 - t5577 + 11.75232_f64 * t127 * t5578 * t411 + t5614;
    t5616
}
