//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1171/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1171<F: Float>(t15375: F, t15380: F, t15384: F, t15389: F, t15391: F, t15393: F, t15397: F, t15399: F, t15401: F, t15403: F, t15405: F, t15407: F) -> F {
    let t15409 = -F::cast_from(0.007556666666666666_f64) * t15375 - F::cast_from(0.02518888888888889_f64) * t15380 + F::cast_from(0.002099074074074074_f64) * t15384 + F::cast_from(0.005597530864197531_f64) * t15389 - F::cast_from(0.007556666666666666_f64) * t15391 + F::cast_from(0.05541555555555556_f64) * t15393 + F::new(0.011335) * t15397 + F::cast_from(0.002518888888888889_f64) * t15399 - F::cast_from(0.0008396296296296296_f64) * t15401 + F::cast_from(0.005037777777777778_f64) * t15403 - F::cast_from(0.0013993827160493828_f64) * t15405 - F::cast_from(0.01847185185185185_f64) * t15407;
    t15409
}
