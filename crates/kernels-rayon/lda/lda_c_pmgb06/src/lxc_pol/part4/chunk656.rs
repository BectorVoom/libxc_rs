//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 656/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk656(t474: f64, t955: f64, t1626: f64, t405: f64, t1620: f64, t134: f64, t443: f64, t147: f64, t135: f64, t146: f64, t3365: f64, t3080: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3396 = t955 * t474;
    let t3398 = t405 * t1626;
    let t3400 = t405 * t1620;
    let t3403 = 1.0_f64 / t134 / t443;
    let t3404 = t147 * t3403;
    let t3413 = 0.02962962962962963_f64 * t146 * t3365 * t135;
    let t3414 = 0.11197407407407407_f64 * t3080;
    (t3396, t3398, t3400, t3403, t3404, t3413, t3414)
}
