//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 737/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk737<F: Float>(t1312: F, t1316: F, t1317: F, t1324: F, t2258: F, t2308: F, t2311: F, t295: F, t312: F, t329: F, t346: F, t384: F, t388: F, t4013: F, t4017: F, t4053: F, t4231: F, t4355: F, t4358: F, t4360: F, t4394: F, t4398: F, t4405: F, t4414: F, t4471: F, t4580: F, t5538: F, t5563: F, t77: F, t770: F, t790: F) -> (F,) {
    let t5566 = -3.0 * t4231 * t4355 + 12.0 * t4358 * t4360 + 3.0 * t329 * t77 * t4394 - 2.0 * t346 * t4398 * t1324 + 2.0 * t346 * t2258 * t384 + 6.0 * t4405 * t770 + 6.0 * t1316 * t2258 * t1317 + 3.0 * t1316 * t790 * t4017 + 3.0 * t1316 * t388 * t4414 - 2.0 * t346 * t2308 * t4013 - t346 * t2308 * t4053 + 3.0 * t1316 * t1312 * t2311 + (t4471 + t4580) * t312 + (t5538 + t5563) * t295;
    (t5566,)
}
