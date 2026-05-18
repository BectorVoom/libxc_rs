//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1025/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1025<F: Float>(t1444: F, t7674: F, t16794: F, t493: F, t834: F, t2010: F, t2011: F, t6123: F, t6119: F, t6286: F, t432: F, t7719: F) -> (F, F, F, F, F) {
    let t19265 = t1444 * t7674 / F::new(15.0);
    let t19268 = t493 * t16794 * t834 / F::new(15.0);
    let t19271 = F::new(2.0) / F::new(15.0) * t2010 * t6123 * t2011;
    let t19274 = F::new(3.0) / F::new(5.0) * t493 * t6119 * t6286;
    let t19276 = t432 * t7719 / F::new(5.0);
    (t19265, t19268, t19271, t19274, t19276)
}
