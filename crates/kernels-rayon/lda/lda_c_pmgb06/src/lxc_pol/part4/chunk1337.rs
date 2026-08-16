//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1337/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1337(t17577: f64, t129: f64, t15844: f64, t1558: f64, t442: f64, t79: f64, t13439: f64, t13452: f64, t1972: f64, t5176: f64, t13502: f64, t13504: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17578 = 8.0_f64 / 135.0_f64 * t17577;
    let t17579 = t129 * t15844;
    let t17583 = 16.0_f64 / 45.0_f64 * t17579 * t442 * t1558 * t79;
    let t17584 = 4.0_f64 / 135.0_f64 * t13439;
    let t17585 = 4.0_f64 / 45.0_f64 * t13452;
    let t17587 = 4.0_f64 / 15.0_f64 * t1972 * t5176;
    let t17588 = 8.0_f64 / 135.0_f64 * t13502;
    let t17589 = 4.0_f64 / 135.0_f64 * t13504;
    (t17578, t17579, t17583, t17584, t17585, t17587, t17588, t17589)
}
