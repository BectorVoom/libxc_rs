//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 798/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk798(t1312: f64, t1316: f64, t1317: f64, t1324: f64, t2258: f64, t2308: f64, t2311: f64, t295: f64, t312: f64, t329: f64, t346: f64, t384: f64, t388: f64, t4013: f64, t4017: f64, t4053: f64, t4231: f64, t4355: f64, t4358: f64, t4360: f64, t4394: f64, t4398: f64, t4405: f64, t4414: f64, t4471: f64, t4580: f64, t5538: f64, t5563: f64, t77: f64, t770: f64, t790: f64) -> f64 {
    let t5566 = -3.0_f64 * t4231 * t4355 + 12.0_f64 * t4358 * t4360 + 3.0_f64 * t329 * t77 * t4394 - 2.0_f64 * t346 * t4398 * t1324 + 2.0_f64 * t346 * t2258 * t384 + 6.0_f64 * t4405 * t770 + 6.0_f64 * t1316 * t2258 * t1317 + 3.0_f64 * t1316 * t790 * t4017 + 3.0_f64 * t1316 * t388 * t4414 - 2.0_f64 * t346 * t2308 * t4013 - t346 * t2308 * t4053 + 3.0_f64 * t1316 * t1312 * t2311 + (t4471 + t4580) * t312 + (t5538 + t5563) * t295;
    t5566
}
