//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1207/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1207<F: Float>(t493: F, t5276: F, t5486: F, t12617: F, t5281: F, t1083: F, t1380: F, t6781: F, t15879: F, t15883: F, t15886: F, t15888: F, t15890: F, t15892: F, t15894: F, t15896: F, t15898: F, t15900: F, t15902: F, t15905: F) -> (F, F, F, F) {
    let t15908 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t5486 * t5276;
    let t15911 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t493 * t12617 * t5281;
    let t15915 = t493 * t1380 * t6781 * t1083 / F::cast_from(45.0_f64);
    let t15916 = t15879 + t15883 + t15886 - t15888 - t15890 - t15892 - t15894 + t15896 - t15898 - t15900 - t15902 - t15905 - t15908 - t15911 - t15915;
    (t15908, t15911, t15915, t15916)
}
