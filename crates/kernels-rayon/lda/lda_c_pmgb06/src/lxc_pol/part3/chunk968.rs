//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 968/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk968(t8306: f64, t8310: f64, t8328: f64, t8341: f64, t8313: f64, t8316: f64, t8324: f64, t8326: f64, t8339: f64, t8346: f64, t8348: f64, t8355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11430 = 3.8973666666666666_f64 * t8306;
    let t11431 = 0.48717083333333333_f64 * t8310;
    let t11436 = 0.9743416666666667_f64 * t8328;
    let t11437 = 2.923025_f64 * t8341;
    let t11439 = -t11430 + t11431 - 2.0_f64 / 3.0_f64 * t8313 + t8316 / 6.0_f64 - 1.46904_f64 * t8324 + 0.73452_f64 * t8326 + t11436 - t8339 + t11437 + t8346 + 14.6904_f64 * t8348;
    let t11441 = 4.5469277777777775_f64 * t8355;
    (t11430, t11431, t11436, t11437, t11439, t11441)
}
