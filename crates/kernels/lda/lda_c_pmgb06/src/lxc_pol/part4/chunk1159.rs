//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1159/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1159<F: Float>(t15248: F, t14310: F, t14312: F, t14314: F, t15215: F, t15217: F, t15222: F, t15226: F, t15230: F, t15233: F, t15236: F, t15238: F, t15243: F, t15245: F, t15247: F) -> (F, F) {
    let t15249 = F::new(8.0) / F::new(135.0) * t15248;
    let t15253 = -t15215 + t15217 - t15222 - t15226 - t15230 + t15233 + t15236 - t15238 + t15243 - t15245 + t15247 - t15249 + F::cast_from(0.2885611029963958_f64) * t14310 + F::cast_from(0.4328416544945937_f64) * t14312 - F::cast_from(0.19237406866426388_f64) * t14314;
    (t15249, t15253)
}
