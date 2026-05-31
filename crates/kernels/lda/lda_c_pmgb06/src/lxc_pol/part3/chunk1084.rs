//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1084/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1084<F: Float>(t1920: F, t3223: F, t2002: F, t2949: F, t2953: F, t2962: F, t5168: F, t5264: F, t1444: F, t5494: F, t2987: F, t493: F, t5486: F) -> (F, F, F, F, F, F, F) {
    let t12878 = t3223 * t1920;
    let t12879 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t12878;
    let t12881 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t2949;
    let t12883 = t2002 * t2953 / F::cast_from(15.0_f64);
    let t12885 = t2002 * t2962 / F::cast_from(9.0_f64);
    let t12887 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5168 * t5264;
    let t12889 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1444 * t5494;
    let t12892 = t493 * t5486 * t2987 / F::cast_from(15.0_f64);
    (t12879, t12881, t12883, t12885, t12887, t12889, t12892)
}
