//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1210/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1210<F: Float>(t13156: F, t13158: F, t13160: F, t13162: F, t13165: F, t13167: F, t13170: F, t13172: F, t13178: F, t13181: F, t13185: F, t13187: F, t13189: F, t13191: F, t13193: F, t13195: F, t13197: F, t13200: F, t13202: F, t13205: F, t13207: F, t13210: F, t13212: F) -> (F, F) {
    let t14412 = t13156 + t13158 + t13160 + t13162 + t13165 + t13167 + t13170 + t13172 + t13178 + t13181 + t13185;
    let t14413 = t13187 + t13189 - t13191 - t13193 - t13195 - t13197 + t13200 + t13202 + t13205 + t13207 + t13210 + t13212;
    (t14412, t14413)
}
