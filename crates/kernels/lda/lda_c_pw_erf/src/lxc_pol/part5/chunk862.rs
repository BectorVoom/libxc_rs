//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 862/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk862<F: Float>(t3706: F, t3917: F, t3919: F, t3923: F, t3929: F, t3935: F, t3938: F, t5806: F, t7519: F, t7524: F, t7530: F, t7531: F, t7532: F, t7534: F, t7536: F, t7538: F, t7540: F, t7541: F) -> F {
    let t8031 = t7519 - t7524 + F::new(2.0) / F::new(45.0) * t5806 - t3706 + t7530 - t7531 - t7532 - t7534 + t7536 + t7538 + t7540 - t7541 + t3917 + t3919 + t3923 + t3929 + t3935 - t3938;
    t8031
}
