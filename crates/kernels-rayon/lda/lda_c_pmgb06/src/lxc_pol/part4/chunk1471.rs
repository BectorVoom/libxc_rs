//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1471/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1471(t395: f64, t6104: f64, t10902: f64, t10934: f64, t10937: f64, t10940: f64, t10943: f64, t10946: f64, t14696: f64, t14699: f64, t14702: f64, t14705: f64) -> f64 {
    let t18979 = t395 * t6104;
    let t18985 = -0.14149184788746388_f64 * t10934 - 0.28298369577492777_f64 * t10937 - 0.14149184788746388_f64 * t10940 + 1.0376068845080684_f64 * t10943 + 1.0376068845080684_f64 * t10946 - 0.2133002709687175_f64 * t18979 - 0.5659673915498555_f64 * t14696 - 0.5659673915498555_f64 * t14699 - 0.5659673915498555_f64 * t14702 - 0.5659673915498555_f64 * t14705 - t10902;
    t18985
}
