//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1357/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1357<F: Float>(t13891: F, t13893: F, t13895: F, t13905: F, t13907: F, t13909: F, t13911: F, t13913: F, t13915: F, t13917: F, t13920: F, t13922: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17842 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13891;
    let t17843 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13893;
    let t17844 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13895;
    let t17845 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13905;
    let t17846 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13907;
    let t17847 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t13909;
    let t17848 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13911;
    let t17849 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13913;
    let t17850 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t13915;
    let t17851 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13917;
    let t17852 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t13920;
    let t17853 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t13922;
    (t17842, t17843, t17844, t17845, t17846, t17847, t17848, t17849, t17850, t17851, t17852, t17853)
}
