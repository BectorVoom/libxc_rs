//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 728/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk728<F: Float>(t4041: F, t5179: F, t5186: F, t5190: F, t5192: F, t5194: F, t5198: F, t5200: F, t6785: F, t6786: F, t6790: F, t6792: F, t6847: F, t6849: F, t6853: F, t6858: F, t6860: F) -> (F,) {
    let t7270 = -t6785 + t5179 - t6786 + t6790 + t4041 + t6792 - t5186 + t5190 + t5192 + t5194 - t5198 + t5200 - t6847 - t6849 + t6853 - t6858 - t6860;
    (t7270,)
}
