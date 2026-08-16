//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1185/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1185<F: Float>(t13602: F, t13604: F, t15375: F, t15380: F, t15384: F, t15389: F, t15391: F, t15393: F, t15397: F, t15399: F, t15401: F, t15403: F) -> F {
    let t15626 = -F::cast_from(0.07111111111111111_f64) * t13602 - F::cast_from(0.017777777777777778_f64) * t13604 + F::cast_from(0.14396666666666666_f64) * t15375 + F::cast_from(0.47988888888888886_f64) * t15380 - F::cast_from(0.03999074074074074_f64) * t15384 - F::cast_from(0.10664197530864197_f64) * t15389 + F::cast_from(0.14396666666666666_f64) * t15391 - F::cast_from(1.0557555555555556_f64) * t15393 - F::cast_from(0.21595_f64) * t15397 - F::cast_from(0.047988888888888886_f64) * t15399 + F::cast_from(0.015996296296296297_f64) * t15401 - F::cast_from(0.09597777777777777_f64) * t15403;
    t15626
}
