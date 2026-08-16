//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1185/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1185(t13602: f64, t13604: f64, t15375: f64, t15380: f64, t15384: f64, t15389: f64, t15391: f64, t15393: f64, t15397: f64, t15399: f64, t15401: f64, t15403: f64) -> f64 {
    let t15626 = -0.07111111111111111_f64 * t13602 - 0.017777777777777778_f64 * t13604 + 0.14396666666666666_f64 * t15375 + 0.47988888888888886_f64 * t15380 - 0.03999074074074074_f64 * t15384 - 0.10664197530864197_f64 * t15389 + 0.14396666666666666_f64 * t15391 - 1.0557555555555556_f64 * t15393 - 0.21595_f64 * t15397 - 0.047988888888888886_f64 * t15399 + 0.015996296296296297_f64 * t15401 - 0.09597777777777777_f64 * t15403;
    t15626
}
