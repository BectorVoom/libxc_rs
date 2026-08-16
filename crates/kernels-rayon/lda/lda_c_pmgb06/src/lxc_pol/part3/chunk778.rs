//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 778/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk778(t500: f64, t5305: f64, t1451: f64, t1972: f64, t1420: f64, t1963: f64, t1835: f64, t495: f64, t499: f64, t493: f64, t1444: f64, t1989: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5307 = 2.0_f64 / 45.0_f64 * t5305 * t500;
    let t5309 = 2.0_f64 / 45.0_f64 * t1972 * t1451;
    let t5311 = 2.0_f64 / 45.0_f64 * t1420 * t1963;
    let t5312 = t495 * t1835;
    let t5313 = t5312 * t499;
    let t5315 = 2.0_f64 / 45.0_f64 * t493 * t5313;
    let t5317 = 2.0_f64 / 45.0_f64 * t1444 * t1989;
    (t5307, t5309, t5311, t5312, t5313, t5315, t5317)
}
