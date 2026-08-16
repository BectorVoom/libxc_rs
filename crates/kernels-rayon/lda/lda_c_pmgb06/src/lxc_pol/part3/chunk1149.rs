//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1149/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1149(t13715: f64, t3262: f64, t439: f64, t1969: f64, t3213: f64, t1423: f64, t4620: f64, t5197: f64, t5202: f64, t1886: f64, t607: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t13718 = 8.0_f64 / 81.0_f64 * t439 * t13715 * t3262;
    let t13719 = t3213 * t1969;
    let t13720 = 2.0_f64 / 45.0_f64 * t13719;
    let t13721 = t1423 * t4620;
    let t13722 = 2.0_f64 / 27.0_f64 * t13721;
    let t13725 = 2.0_f64 / 5.0_f64 * t439 * t5197 * t5202;
    let t13726 = t1886 * t607;
    let t13727 = t13726 * t446;
    (t13718, t13720, t13722, t13725, t13727)
}
