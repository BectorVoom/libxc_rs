//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1082/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1082<F: Float>(t2965: F, t439: F, t5482: F, t1444: F, t5451: F, t5454: F, t3459: F, t493: F, t838: F, t9908: F, t2912: F, t4856: F) -> (F, F, F, F, F) {
    let t12855 = F::new(2.0) / F::new(15.0) * t439 * t5482 * t2965;
    let t12857 = F::new(2.0) / F::new(15.0) * t1444 * t5451;
    let t12859 = F::new(2.0) / F::new(3.0) * t1444 * t5454;
    let t12863 = F::new(2.0) / F::new(15.0) * t493 * t9908 * t838 * t3459;
    let t12864 = t4856 * t2912;
    (t12855, t12857, t12859, t12863, t12864)
}
