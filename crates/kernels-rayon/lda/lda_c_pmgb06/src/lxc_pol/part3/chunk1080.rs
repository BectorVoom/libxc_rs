//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1080/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1080(t12831: f64, t9760: f64, t9762: f64, t9765: f64, t9771: f64, t12823: f64, t12824: f64, t12826: f64, t12827: f64, t12829: f64, t9759: f64, t9770: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12832 = t12831 / 45.0_f64;
    let t12833 = t9760 / 15.0_f64;
    let t12834 = 4.0_f64 / 135.0_f64 * t9762;
    let t12835 = 4.0_f64 / 135.0_f64 * t9765;
    let t12836 = t9771 / 15.0_f64;
    let t12837 = -t12823 - t12824 - t12826 + t12827 + t9759 - t12829 - t12832 + t12833 + t12834 + t12835 - t9770 + t12836;
    (t12832, t12833, t12834, t12835, t12836, t12837)
}
