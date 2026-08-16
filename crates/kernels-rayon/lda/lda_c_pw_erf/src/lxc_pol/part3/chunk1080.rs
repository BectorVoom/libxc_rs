//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1080/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1080(t558: f64, t581: f64, t2151: f64, t571: f64, t593: f64, t12622: f64, t12624: f64, t12626: f64, t12628: f64, t12630: f64, t12632: f64, t12634: f64, t12636: f64, t12638: f64, t12640: f64, t12643: f64, t12645: f64) -> (f64, f64) {
    let t12646 = t581 * t558;
    let t12650 = 8.0_f64 / 15.0_f64 * t571 * t2151 * t12646 * t593;
    let t12651 = -t12622 - t12624 - t12626 - t12628 + t12630 + t12632 - t12634 + t12636 - t12638 - t12640 - t12643 + t12645 + t12650;
    (t12650, t12651)
}
