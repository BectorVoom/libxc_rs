//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1341/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1341(t22210: f64, t22212: f64, t22214: f64, t22216: f64, t22218: f64, t22222: f64, t22225: f64, t22228: f64, t22231: f64, t22234: f64, t22237: f64, t22239: f64, t22243: f64) -> f64 {
    let t23296 = t22210 + t22212 + t22214 + t22216 + t22218 - t22222 - t22225 + t22228 + t22231 - t22234 + t22237 - t22239 - t22243;
    t23296
}
