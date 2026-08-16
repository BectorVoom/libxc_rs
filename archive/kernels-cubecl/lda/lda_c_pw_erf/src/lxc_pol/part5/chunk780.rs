//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 780/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk780<F: Float>(t4468: F, t4470: F, t6161: F, t6192: F, t6197: F, t6200: F, t6202: F, t6204: F, t6207: F, t6211: F, t6213: F, t6217: F, t6219: F, t6222: F, t6224: F, t6228: F) -> F {
    let t7238 = -t6161 + t4468 + t4470 - t6192 + t6197 + t6200 - t6202 + t6204 + t6207 + t6211 + t6213 + t6217 + t6219 + t6222 - t6224 + t6228;
    t7238
}
