//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 310/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk310(t1112: f64, t691: f64, t638: f64, t654: f64, t687: f64, t258: f64, t30: f64) -> (f64, f64, f64, f64) {
    let t1114 = 0.00024415263074675396_f64 * t691 * t1112;
    let t1115 = t638 * t654;
    let t1118 = 8.0_f64 * t638 * t687;
    let t1121 = t258 * t30;
    (t1114, t1115, t1118, t1121)
}
