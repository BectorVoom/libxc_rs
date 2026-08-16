//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1142/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1142<F: Float>(t12329: F, t12337: F, t12366: F, t12369: F, t15391: F, t15393: F, t15399: F, t15401: F, t15403: F, t15405: F, t15407: F, t15416: F, t15418: F, t15423: F, t15435: F, t19918: F, t19920: F, t19922: F, t19925: F, t9178: F) -> F {
    let t20734 = F::cast_from(0.005037777777777778_f64) * t12329 - F::cast_from(0.005877407407407408_f64) * t12337 - F::cast_from(0.005037777777777778_f64) * t12366 + t12369 - F::cast_from(0.011335_f64) * t15391 + F::cast_from(0.015113333333333333_f64) * t15393 + F::cast_from(0.003778333333333333_f64) * t15399 - F::cast_from(0.0012594444444444445_f64) * t15401 + F::cast_from(0.007556666666666666_f64) * t15403 - F::cast_from(0.002099074074074074_f64) * t15405 - F::cast_from(0.005037777777777778_f64) * t15407 - t9178 + F::cast_from(0.002518888888888889_f64) * t15416 + F::cast_from(0.0016792592592592592_f64) * t15418 - F::cast_from(0.005037777777777778_f64) * t15423 - F::cast_from(0.0018891666666666666_f64) * t15435 + F::cast_from(0.034005_f64) * t19918 + F::cast_from(0.002518888888888889_f64) * t19920 - F::cast_from(0.003778333333333333_f64) * t19922 + F::cast_from(0.0018891666666666666_f64) * t19925;
    t20734
}
