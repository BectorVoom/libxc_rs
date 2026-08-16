//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 360/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk360(t1555: f64, t161: f64, t153: f64, t1531: f64, t1472: f64, t147: f64, t315: f64, t146: f64, t164: f64, t405: f64, t526: f64, t162: f64, t163: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1557 = t161 * t1555 / 135.0_f64;
    let t1558 = t153 * t1531;
    let t1563 = 0.047988888888888886_f64 * t1472;
    let t1568 = t315 * t147;
    let t1571 = 0.011111111111111112_f64 * t146 * t1568 * t164;
    let t1572 = t405 * t526;
    let t1575 = 1.0_f64 / t163 / t162;
    (t1557, t1558, t1563, t1568, t1571, t1572, t1575)
}
