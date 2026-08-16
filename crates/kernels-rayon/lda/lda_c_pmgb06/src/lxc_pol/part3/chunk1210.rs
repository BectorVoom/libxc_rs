//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1210/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1210(t13156: f64, t13158: f64, t13160: f64, t13162: f64, t13165: f64, t13167: f64, t13170: f64, t13172: f64, t13178: f64, t13181: f64, t13185: f64, t13187: f64, t13189: f64, t13191: f64, t13193: f64, t13195: f64, t13197: f64, t13200: f64, t13202: f64, t13205: f64, t13207: f64, t13210: f64, t13212: f64) -> (f64, f64) {
    let t14412 = t13156 + t13158 + t13160 + t13162 + t13165 + t13167 + t13170 + t13172 + t13178 + t13181 + t13185;
    let t14413 = t13187 + t13189 - t13191 - t13193 - t13195 - t13197 + t13200 + t13202 + t13205 + t13207 + t13210 + t13212;
    (t14412, t14413)
}
