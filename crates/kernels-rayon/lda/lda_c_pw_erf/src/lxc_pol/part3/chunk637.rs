//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 637/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk637(t3762: f64, t576: f64, t571: f64, t1469: f64, t3416: f64, t1287: f64, t581: f64, t593: f64, t1466: f64, t1318: f64, t1278: f64, t529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3763 = t3762 * t576;
    let t3764 = t571 * t3763;
    let t3765 = 8.0_f64 / 135.0_f64 * t3764;
    let t3767 = 8.0_f64 / 5.0_f64 * t3416 * t1469;
    let t3769 = t581 * t1287 * t593;
    let t3770 = t1466 * t3769;
    let t3772 = 4.0_f64 / 5.0_f64 * t1318 * t3770;
    let t3773 = t529 * t1278;
    (t3763, t3764, t3765, t3767, t3769, t3770, t3772, t3773)
}
