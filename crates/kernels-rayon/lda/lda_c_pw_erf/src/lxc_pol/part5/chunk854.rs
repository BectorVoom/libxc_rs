//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 854/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk854(t1697: f64, t7918: f64, t10: f64, t3280: f64, t3282: f64, t3284: f64, t3288: f64, t3290: f64, t426: f64, t5502: f64, t5507: f64, t5513: f64, t7893: f64, t7896: f64, t7897: f64, t7915: f64) -> (f64, f64, f64) {
    let t7919 = t1697 * t7918;
    let t7920 = t10 * t7919;
    let t7923 = -2.93808_f64 * t5502 - t7893 - 2.0_f64 / 3.0_f64 * t5507 - 1.46904_f64 * t5513 + t7896 + t3280 - t3282 - t3284 - t3288 - t3290 + 9.0_f64 / 2.0_f64 * t426 * t10 * t7897 - t426 * t7915 / 2.0_f64 - 6.0_f64 * t426 * t7920;
    (t7919, t7920, t7923)
}
