//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 736/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk736<F: Float>(t4406: F, t4408: F, t4412: F, t6056: F, t2848: F, t2850: F, t2852: F) -> (F, F, F, F, F) {
    let t7330 = 3.0 * t4406;
    let t7332 = 60.0 * t4408;
    let t7333 = 3.5089340384731225 * t4412;
    let t7336 = 0.0005493466511025948 * t6056;
    let t7337 = t2848 + t2850 + t2852;
    (t7330, t7332, t7333, t7336, t7337)
}
