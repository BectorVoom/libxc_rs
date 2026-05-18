//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1122/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1122<F: Float>(t1972: F, t6518: F, t6783: F, t2002: F, t6499: F, t153: F, t1864: F, t439: F, t6123: F, t16118: F, t1859: F, t16866: F) -> (F, F, F, F, F, F) {
    let t20478 = F::new(2.0) / F::new(15.0) * t1972 * t6518;
    let t20480 = t1972 * t6783 / F::new(15.0);
    let t20482 = F::new(2.0) / F::new(9.0) * t2002 * t6499;
    let t20486 = F::new(2.0) / F::new(15.0) * t439 * t6123 * t153 * t1864;
    let t20490 = t439 * t16118 * t153 * t1859 / F::new(9.0);
    let t20491 = F::new(2.0) / F::new(135.0) * t16866;
    (t20478, t20480, t20482, t20486, t20490, t20491)
}
