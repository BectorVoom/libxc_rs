//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1159/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1159(t15248: f64, t14310: f64, t14312: f64, t14314: f64, t15215: f64, t15217: f64, t15222: f64, t15226: f64, t15230: f64, t15233: f64, t15236: f64, t15238: f64, t15243: f64, t15245: f64, t15247: f64) -> (f64, f64) {
    let t15249 = 8.0_f64 / 135.0_f64 * t15248;
    let t15253 = -t15215 + t15217 - t15222 - t15226 - t15230 + t15233 + t15236 - t15238 + t15243 - t15245 + t15247 - t15249 + 0.2885611029963958_f64 * t14310 + 0.4328416544945937_f64 * t14312 - 0.19237406866426388_f64 * t14314;
    (t15249, t15253)
}
