//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1077/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1077<F: Float>(t142: F, t7913: F, t455: F, t159: F, t285: F, t462: F, t7337: F, t11635: F, t8777: F) -> (F, F, F, F) {
    let t20143 = t142 * t7913;
    let t20144 = t455 * t20143;
    let t20174 = t462 * t7337 * t159 * t285;
    let t20179 = -t11635 - t8777;
    (t20143, t20144, t20174, t20179)
}
