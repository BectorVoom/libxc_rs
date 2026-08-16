//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 697/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk697(t439: f64, t6365: f64, t224: f64, t6308: f64, t6310: f64, t6312: f64, t6314: f64, t6316: f64, t6318: f64, t6320: f64, t6322: f64, t6324: f64, t6326: f64, t6327: f64, t6355: f64, t6358: f64, t6360: f64, t6363: f64) -> (f64, f64) {
    let t6367 = 2.0_f64 / 45.0_f64 * t439 * t6365;
    let t6368 = -t6308 - t6310 + t6312 + t6314 + t6316 + t6318 + t6320 + t6322 + t6324 + t6326 + 2.0_f64 / 9.0_f64 * t6327 - t6355 * t224 / 15.0_f64 - 2.0_f64 / 45.0_f64 * t6358 - t6360 - t6363 - t6367;
    (t6367, t6368)
}
