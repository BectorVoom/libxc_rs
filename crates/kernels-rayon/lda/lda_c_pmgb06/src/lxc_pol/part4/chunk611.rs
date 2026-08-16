//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 611/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk611(t1296: f64, t2238: f64, t2718: f64, t2722: f64, t2730: f64, t378: f64, t74: f64, t787: f64, t387: f64) -> (f64, f64) {
    let t2732 = 2.0_f64 * t1296 * t2722 - 2.0_f64 * t2238 * t787 + t2718 * t74 - t378 * t2730;
    let t2733 = t2732 * t387;
    (t2732, t2733)
}
