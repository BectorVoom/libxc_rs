//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 595/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk595(t2623: f64, t518: f64, t166: f64, t161: f64, t831: f64, t853: f64, t2106: f64, t822: f64, t137: f64, t132: f64, t1619: f64, t2570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2624 = t518 * t2623;
    let t2625 = t166 * t2624;
    let t2627 = t161 * t2625 / 30.0_f64;
    let t2629 = t831 * t853 / 15.0_f64;
    let t2630 = t2106 * t822;
    let t2631 = t137 * t2630;
    let t2633 = t132 * t2631 / 15.0_f64;
    let t2639 = t1619 * t2570;
    (t2624, t2625, t2627, t2629, t2630, t2631, t2633, t2639)
}
