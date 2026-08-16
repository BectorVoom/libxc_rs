//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1095/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1095(t16491: f64, t439: f64, t6151: f64, t12753: f64, t20146: f64, t20151: f64, t20155: f64, t20159: f64, t20161: f64, t20162: f64, t20165: f64, t20168: f64, t20171: f64, t20174: f64) -> (f64, f64) {
    let t20177 = 8.0_f64 / 27.0_f64 * t439 * t16491 * t6151;
    let t20178 = t20146 + t20151 - t20155 + t20159 - t20161 - t20162 + t12753 - t20165 - t20168 + t20171 - t20174 + t20177;
    (t20177, t20178)
}
