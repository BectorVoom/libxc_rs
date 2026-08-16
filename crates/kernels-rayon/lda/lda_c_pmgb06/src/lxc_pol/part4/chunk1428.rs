//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1428/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1428(t13444: f64, t13447: f64, t13450: f64, t17561: f64, t17564: f64, t17571: f64, t17575: f64, t17576: f64, t17578: f64, t17583: f64, t17584: f64, t17585: f64, t17587: f64, t17588: f64, t17589: f64) -> f64 {
    let t18314 = t17561 - t17564 + t17571 - t17575 + t17576 - t17578 - t17583 + t17584 + 4.0_f64 / 3.0_f64 * t13444 + 2.0_f64 / 3.0_f64 * t13447 + 0.36466666666666664_f64 * t13450 + t17585 + t17587 + t17588 + t17589;
    t18314
}
