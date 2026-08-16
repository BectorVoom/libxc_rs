//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1037/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1037(t161: f64, t166: f64, t176: f64, t19375: f64, t19414: f64, t1848: f64, t2555: f64, t6833: f64, t831: f64, t15519: f64, t15521: f64, t15523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19419 = t161 * t166 * (t19375 + t19414) * t176 / 30.0_f64;
    let t19421 = t1848 * t2555 / 10.0_f64;
    let t19423 = t831 * t6833 / 10.0_f64;
    let t19424 = 4.0_f64 / 45.0_f64 * t15519;
    let t19425 = 8.0_f64 / 45.0_f64 * t15521;
    let t19426 = 4.0_f64 / 27.0_f64 * t15523;
    (t19419, t19421, t19423, t19424, t19425, t19426)
}
