//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1155/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1155(t15208: f64, t15173: f64, t15176: f64, t15179: f64, t15181: f64, t15183: f64, t15185: f64, t15188: f64, t15190: f64, t15195: f64, t15197: f64, t15199: f64, t15203: f64, t15207: f64) -> (f64, f64) {
    let t15209 = 8.0_f64 / 135.0_f64 * t15208;
    let t15210 = t15173 + t15176 + t15179 - t15181 - t15183 - t15185 + t15188 - t15190 + t15195 - t15197 + t15199 + t15203 + t15207 + t15209;
    (t15209, t15210)
}
