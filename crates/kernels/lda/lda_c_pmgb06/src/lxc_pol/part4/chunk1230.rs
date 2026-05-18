//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1230/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1230<F: Float>(t1420: F, t6472: F, t439: F, t5253: F, t6146: F, t15373: F, t1901: F, t15378: F, t5260: F, t5474: F, t6268: F, t1894: F, t5220: F) -> (F, F, F, F, F, F) {
    let t16201 = F::new(4.0) / F::new(9.0) * t1420 * t6472;
    let t16204 = F::new(4.0) / F::new(9.0) * t439 * t5253 * t6146;
    let t16207 = F::new(2.0) / F::new(9.0) * t439 * t1901 * t15373;
    let t16210 = F::new(32.0) / F::new(27.0) * t439 * t5260 * t15378;
    let t16212 = F::new(8.0) / F::new(27.0) * t6268 * t5474;
    let t16213 = t5220 * t1894;
    (t16201, t16204, t16207, t16210, t16212, t16213)
}
