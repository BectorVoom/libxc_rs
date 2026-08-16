//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1167/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1167(t10439: f64, t21022: f64, t332: f64, t439: f64, t2002: f64, t6413: f64, t132: f64, t435: f64, t7862: f64, t1897: f64, t19782: f64, t2010: f64) -> (f64, f64, f64, f64) {
    let t21026 = 2.0_f64 / 15.0_f64 * t439 * t10439 * t21022 * t332;
    let t21028 = t2002 * t6413 / 15.0_f64;
    let t21032 = t132 * t435 * t7862;
    let t21033 = t21032 / 15.0_f64;
    let t21036 = 4.0_f64 / 15.0_f64 * t2010 * t1897 * t19782;
    (t21026, t21028, t21033, t21036)
}
