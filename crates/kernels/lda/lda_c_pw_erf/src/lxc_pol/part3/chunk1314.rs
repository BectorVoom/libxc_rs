//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1314/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1314<F: Float>(t1427: F, t5795: F, t11636: F, t14074: F, t14076: F, t14078: F, t14083: F, t14088: F, t14090: F, t15123: F, t15125: F, t15132: F, t15135: F, t15138: F, t247: F, t251: F, t256: F) -> F {
    let t15139 = t5795 * t1427;
    let t15140 = F::new(0.36466666666666664) * t15139;
    let t15141 = t14074 + t14076 - t14078 + t15123 / F::new(3.0) + F::new(0.18233333333333332) * t15125 + t11636 * t247 * t251 * t256 / F::new(3.0) + t15132 + F::new(0.18233333333333332) * t15135 + t15138 + t15140 - t14083 - t14088 + t14090;
    t15141
}
