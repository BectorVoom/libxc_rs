//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1193/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1193(t18848: f64, t18851: f64, t21411: f64, t21439: f64, t21442: f64, t21445: f64, t21448: f64, t21451: f64, t21461: f64, t21462: f64, t21463: f64, t21465: f64, t21466: f64, t21477: f64, t2247: f64) -> f64 {
    let t21595 = 20.69106_f64 * t18848 - 10.34553_f64 * t18851 - 62.07318_f64 * t2247 * t21411 + t21439 - t21442 + t21445 + t21448 + t21451 - t21461 + t21462 + t21463 + t21465 - t21466 - t21477;
    t21595
}
