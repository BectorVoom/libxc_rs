//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 972/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk972<F: Float>(t13192: F, t2880: F, t831: F, t4612: F, t5211: F, t1983: F, t485: F, t5210: F, t5322: F, t5499: F, t13178: F, t13181: F, t13185: F, t13187: F, t13189: F, t13191: F) -> (F, F, F, F, F, F) {
    let t13193 = 2.0 / 15.0 * t13192;
    let t13194 = t831 * t2880;
    let t13195 = 2.0 / 15.0 * t13194;
    let t13196 = t5211 * t4612;
    let t13197 = 2.0 / 9.0 * t13196;
    let t13199 = t485 * t5210 * t1983;
    let t13200 = 2.0 / 9.0 * t13199;
    let t13201 = t5499 * t5322;
    let t13202 = 2.0 / 9.0 * t13201;
    let t13203 = t13178 + t13181 + t13185 + t13187 + t13189 - t13191 - t13193 - t13195 - t13197 + t13200 + t13202;
    (t13193, t13195, t13197, t13200, t13202, t13203)
}
