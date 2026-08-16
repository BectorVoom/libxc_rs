//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1167/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1167(t656: f64, t8033: f64, t17040: f64, t12956: f64, t1995: f64, t3965: f64, t4495: f64, t2146: f64, t6970: f64, t6974: f64, t1475: f64, t571: f64, t7478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21336 = t8033 * t656;
    let t21338 = 16.0_f64 / 135.0_f64 * t17040;
    let t21342 = 32.0_f64 / 15.0_f64 * t3965 * t12956 * t1995 * t4495;
    let t21344 = 12.0_f64 / 5.0_f64 * t2146 * t6970;
    let t21346 = 8.0_f64 / 5.0_f64 * t2146 * t6974;
    let t21348 = t571 * t1475 * t7478;
    (t21336, t21338, t21342, t21344, t21346, t21348)
}
