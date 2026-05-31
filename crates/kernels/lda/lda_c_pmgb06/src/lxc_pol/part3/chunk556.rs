//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 556/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk556<F: Float>(t2911: F, t2912: F, t2909: F, t36: F, t12: F, t1463: F) -> (F, F, F, F) {
    let t2913 = t2911 * t2912;
    let t2914 = t2909 * t2913;
    let t2915 = t36 * t2914;
    let t2918 = F::cast_from(1.0_f64) / t1463 / t12;
    (t2913, t2914, t2915, t2918)
}
