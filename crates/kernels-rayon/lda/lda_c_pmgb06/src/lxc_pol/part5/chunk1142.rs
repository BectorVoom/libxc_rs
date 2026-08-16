//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1142/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1142(t12329: f64, t12337: f64, t12366: f64, t12369: f64, t15391: f64, t15393: f64, t15399: f64, t15401: f64, t15403: f64, t15405: f64, t15407: f64, t15416: f64, t15418: f64, t15423: f64, t15435: f64, t19918: f64, t19920: f64, t19922: f64, t19925: f64, t9178: f64) -> f64 {
    let t20734 = 0.005037777777777778_f64 * t12329 - 0.005877407407407408_f64 * t12337 - 0.005037777777777778_f64 * t12366 + t12369 - 0.011335_f64 * t15391 + 0.015113333333333333_f64 * t15393 + 0.003778333333333333_f64 * t15399 - 0.0012594444444444445_f64 * t15401 + 0.007556666666666666_f64 * t15403 - 0.002099074074074074_f64 * t15405 - 0.005037777777777778_f64 * t15407 - t9178 + 0.002518888888888889_f64 * t15416 + 0.0016792592592592592_f64 * t15418 - 0.005037777777777778_f64 * t15423 - 0.0018891666666666666_f64 * t15435 + 0.034005_f64 * t19918 + 0.002518888888888889_f64 * t19920 - 0.003778333333333333_f64 * t19922 + 0.0018891666666666666_f64 * t19925;
    t20734
}
