//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1073/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1073(t19870: f64, t443: f64, t405: f64, t7782: f64, t7788: f64, t7775: f64, t103: f64, t15391: f64, t15393: f64, t15399: f64, t15401: f64, t15403: f64, t15405: f64, t15407: f64, t15591: f64, t15593: f64, t15601: f64, t15603: f64, t1619: f64, t19770: f64, t19778: f64, t19786: f64, t19868: f64, t2060: f64, t473: f64, t9715: f64) -> (f64, f64) {
    let t19871 = t443 * t19870;
    let t19882 = t405 * t7782;
    let t19884 = t405 * t7788;
    let t19886 = t405 * t7775;
    let t19897 = -0.02666666666666667_f64 * t15591 - 0.10666666666666667_f64 * t15593 - 0.02666666666666667_f64 * t15601 + 0.0044444444444444444_f64 * t15603 + 0.0044444444444444444_f64 * t19868 - 0.006666666666666667_f64 * t103 * t473 * t19871 + t9715 + 0.21595_f64 * t15391 - 0.2879333333333333_f64 * t15393 - 0.07198333333333333_f64 * t15399 + 0.023994444444444443_f64 * t15401 - 0.14396666666666666_f64 * t15403 + 0.03999074074074074_f64 * t15405 + 0.09597777777777777_f64 * t15407 - 0.008888888888888889_f64 * t19882 + 0.0019753086419753087_f64 * t19884 + 0.02666666666666667_f64 * t19886 - 0.08_f64 * t103 * t1619 * t19786 - 0.24_f64 * t2060 * t473 * t19770 + 0.04_f64 * t103 * t473 * t19778;
    (t19871, t19897)
}
