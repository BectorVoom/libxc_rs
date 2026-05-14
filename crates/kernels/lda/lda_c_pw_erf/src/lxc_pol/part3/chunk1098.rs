//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1098/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1098<F: Float>(t12367: F, t12369: F, t12372: F, t12376: F, t12383: F, t12386: F, t12392: F, t12395: F, t12398: F, t12402: F, t12406: F, t12408: F, t12410: F, t12412: F, t12416: F, t12420: F, t12423: F, t12427: F, t12432: F, t12435: F, t12438: F, t12442: F, t12444: F, t12449: F, t12453: F, t12456: F, t12459: F) -> (F, F) {
    let t15008 = t12367 - t12369 - t12372 - t12376 - t12383 - t12386 + t12392 + t12395 + t12398 + t12402 - t12406 - t12408 - t12410;
    let t15009 = t12412 + t12416 + t12420 + t12423 + t12427 + t12432 + t12435 + t12438 + t12442 + t12444 + t12449 + t12453 + t12456 + t12459;
    (t15008, t15009)
}
