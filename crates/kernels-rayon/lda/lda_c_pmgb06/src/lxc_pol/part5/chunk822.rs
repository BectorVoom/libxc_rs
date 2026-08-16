//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 822/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk822(t6627: f64, t6710: f64, t473: f64, t7489: f64, t103: f64, t3413: f64, t3414: f64, t4635: f64, t5003: f64, t6205: f64, t6207: f64, t6209: f64, t6211: f64, t6213: f64, t6215: f64) -> (f64, f64, f64, f64) {
    let t7765 = 2.0_f64 / 15.0_f64 * t6627;
    let t7766 = 2.0_f64 / 15.0_f64 * t6710;
    let t7775 = t473 * t7489;
    let t7778 = 0.023994444444444443_f64 * t6205 - 0.07198333333333333_f64 * t6207 + 0.035991666666666665_f64 * t6209 - 0.02666666666666667_f64 * t6211 + 0.013333333333333334_f64 * t6213 + 0.0044444444444444444_f64 * t6215 - t3413 - t3414 - 0.022222222222222223_f64 * t5003 - 0.047988888888888886_f64 * t4635 - 0.04_f64 * t103 * t7775;
    (t7765, t7766, t7775, t7778)
}
