//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 720/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk720<F: Float>(t500: F, t5305: F, t1451: F, t1972: F, t1420: F, t1963: F, t1835: F, t495: F, t499: F, t493: F, t1444: F, t1989: F, t1586: F, t1993: F, t1992: F, t1450: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5307 = 2.0 / 45.0 * t5305 * t500;
    let t5309 = 2.0 / 45.0 * t1972 * t1451;
    let t5311 = 2.0 / 45.0 * t1420 * t1963;
    let t5312 = t495 * t1835;
    let t5313 = t5312 * t499;
    let t5315 = 2.0 / 45.0 * t493 * t5313;
    let t5317 = 2.0 / 45.0 * t1444 * t1989;
    let t5318 = t1993 * t1586;
    let t5319 = t1992 * t5318;
    let t5321 = t493 * t5319 / 15.0;
    let t5322 = t1450 * t1982;
    (t5307, t5309, t5311, t5312, t5313, t5315, t5317, t5318, t5319, t5321, t5322)
}
