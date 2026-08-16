//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1073/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1073<F: Float>(t19870: F, t443: F, t405: F, t7782: F, t7788: F, t7775: F, t103: F, t15391: F, t15393: F, t15399: F, t15401: F, t15403: F, t15405: F, t15407: F, t15591: F, t15593: F, t15601: F, t15603: F, t1619: F, t19770: F, t19778: F, t19786: F, t19868: F, t2060: F, t473: F, t9715: F) -> (F, F) {
    let t19871 = t443 * t19870;
    let t19882 = t405 * t7782;
    let t19884 = t405 * t7788;
    let t19886 = t405 * t7775;
    let t19897 = -F::cast_from(0.02666666666666667_f64) * t15591 - F::cast_from(0.10666666666666667_f64) * t15593 - F::cast_from(0.02666666666666667_f64) * t15601 + F::cast_from(0.0044444444444444444_f64) * t15603 + F::cast_from(0.0044444444444444444_f64) * t19868 - F::cast_from(0.006666666666666667_f64) * t103 * t473 * t19871 + t9715 + F::cast_from(0.21595_f64) * t15391 - F::cast_from(0.2879333333333333_f64) * t15393 - F::cast_from(0.07198333333333333_f64) * t15399 + F::cast_from(0.023994444444444443_f64) * t15401 - F::cast_from(0.14396666666666666_f64) * t15403 + F::cast_from(0.03999074074074074_f64) * t15405 + F::cast_from(0.09597777777777777_f64) * t15407 - F::cast_from(0.008888888888888889_f64) * t19882 + F::cast_from(0.0019753086419753087_f64) * t19884 + F::cast_from(0.02666666666666667_f64) * t19886 - F::cast_from(0.08_f64) * t103 * t1619 * t19786 - F::cast_from(0.24_f64) * t2060 * t473 * t19770 + F::cast_from(0.04_f64) * t103 * t473 * t19778;
    (t19871, t19897)
}
