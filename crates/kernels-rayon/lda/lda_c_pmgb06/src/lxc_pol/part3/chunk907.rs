//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 907/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk907(t1583: f64, t955: f64, t1577: f64, t3362: f64, t405: f64, t3359: f64, t1414: f64, t147: f64, t163: f64, t3338: f64, t146: f64, t164: f64, t9712: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9956 = t955 * t1583;
    let t9958 = t955 * t1577;
    let t9960 = t405 * t3362;
    let t9962 = t405 * t3359;
    let t9967 = t147 / t163 / t1414;
    let t9974 = t405 * t3338;
    let t9981 = 0.10864197530864197_f64 * t146 * t9712 * t164;
    (t9956, t9958, t9960, t9962, t9967, t9974, t9981)
}
