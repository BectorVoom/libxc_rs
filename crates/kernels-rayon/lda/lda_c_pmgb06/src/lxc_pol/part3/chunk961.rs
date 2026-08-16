//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 961/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk961(t30: f64, t342: f64, t410: f64, t5783: f64, t110: f64, t360: f64, t5766: f64, t5770: f64, t5779: f64, t1234: f64, t348: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11316 = t30 * t410 * t342;
    let t11317 = t5783 * t11316;
    let t11318 = 3.8973666666666666_f64 * t11317;
    let t11320 = t360 * t110 * t5766;
    let t11322 = t5770 * t11316;
    let t11323 = 11.75232_f64 * t11322;
    let t11330 = t360 * t110 * t5779;
    let t11334 = t30 * t110 * t1234;
    let t11335 = t348 * t780 * t11334;
    (t11318, t11320, t11323, t11330, t11334, t11335)
}
