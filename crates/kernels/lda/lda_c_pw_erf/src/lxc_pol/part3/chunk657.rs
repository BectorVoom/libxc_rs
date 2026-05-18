//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 657/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk657<F: Float>(t3953: F, t519: F, t1498: F, t568: F, t646: F, t695: F, t1198: F, t1426: F, t458: F, t108: F, t492: F, t267: F) -> (F, F, F, F, F, F, F, F) {
    let t3955 = F::new(4.0) / F::new(9.0) * t519 * t3953;
    let t3956 = t1498 * t568;
    let t3957 = F::new(4.0) / F::new(15.0) * t3956;
    let t3959 = F::new(0.06649088888888889) * t695 * t646;
    let t3960 = t1198 * t646;
    let t3963 = F::new(0.09973633333333333) * t458 * t1426;
    let t3964 = t492 * t108;
    let t3965 = t3964 * t267;
    (t3955, t3956, t3957, t3959, t3960, t3963, t3964, t3965)
}
