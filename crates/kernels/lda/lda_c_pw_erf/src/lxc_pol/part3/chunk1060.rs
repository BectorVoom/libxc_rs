//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1060/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1060<F: Float>(t11334: F, t11336: F, t11338: F, t11340: F, t11341: F, t11342: F, t11343: F, t11344: F, t11345: F, t11346: F, t11347: F, t11349: F, t8202: F, t8221: F, t8224: F, t8238: F, t8244: F) -> (F,) {
    let t14415 = -t11334 - t11336 - t11338 + t11340 + t11341 + t11342 - t8202 + t11343 + t11344 + t11345 - t11346 - t11347 + t11349 - t8221 + t8224 + t8238 - t8244;
    (t14415,)
}
