//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 634/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk634<F: Float>(t3228: F, t3230: F, t3232: F, t3234: F, t3237: F, t3241: F, t3245: F, t3253: F, t3257: F, t3265: F, t3267: F, t3271: F, t3273: F, t3275: F, t3278: F, t3282: F) -> (F,) {
    let t4158 = t3228 + t3230 + t3232 + t3234 + t3237 + t3241 + t3245 + t3253 + t3257 + t3265 + t3267 + t3271 + t3273 + t3275 + t3278 + t3282;
    (t4158,)
}
