//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 939/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk939<F: Float>(t12414: F, t4509: F, t1309: F, t2065: F, t4506: F, t4507: F, t3824: F, t4508: F, t12383: F, t12386: F, t12392: F, t12395: F, t12398: F, t12402: F, t12406: F, t12408: F, t12410: F, t12412: F) -> (F, F, F, F) {
    let t12416 = 16.0 / 15.0 * t12414 * t4509;
    let t12420 = 16.0 / 15.0 * t4506 * t4507 * t2065 * t1309;
    let t12423 = 8.0 / 15.0 * t4506 * t4508 * t3824;
    let t12424 = -t12383 - t12386 + t12392 + t12395 + t12398 + t12402 - t12406 - t12408 - t12410 + t12412 + t12416 + t12420 + t12423;
    (t12416, t12420, t12423, t12424)
}
