//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1359/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1359<F: Float>(t13973: F, t187: F, t7209: F, t7179: F, t13984: F, t4754: F, t824: F, t1887: F, t2043: F, t1491: F, t2563: F, t161: F, t489: F, t6595: F) -> (F, F, F, F, F, F, F, F) {
    let t17858 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13973;
    let t17859 = t7209 * t187;
    let t17861 = t7179 * t187;
    let t17863 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13984;
    let t17869 = t4754 * t824 / F::cast_from(15.0_f64);
    let t17871 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1887 * t2043;
    let t17873 = t2563 * t1491 / F::cast_from(30.0_f64);
    let t17875 = t161 * t489 * t6595;
    (t17858, t17859, t17861, t17863, t17869, t17871, t17873, t17875)
}
