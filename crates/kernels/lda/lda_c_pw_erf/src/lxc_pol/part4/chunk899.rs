//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 899/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk899<F: Float>(t4185: F, t4190: F, t4193: F, t4198: F, t4201: F, t4202: F, t4206: F, t4209: F, t4544: F, t4547: F, t4583: F, t6316: F, t6317: F, t6318: F, t6319: F, t6320: F, t6321: F) -> (F,) {
    let t7248 = 0.1442805514981979 * t4544 + 0.022363485482220676 * t4547 - t6316 - t6317 - t4185 + 0.21642082724729686 * t4190 + 0.011181742741110338 * t4193 + t4198 + t4201 + 0.07214027574909895 * t4202 + t4206 - t4209 + t6318 - t6319 + t6320 - t6321 + t4583;
    (t7248,)
}
