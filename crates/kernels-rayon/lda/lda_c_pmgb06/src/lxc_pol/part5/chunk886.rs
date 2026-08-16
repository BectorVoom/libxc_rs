//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 886/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk886(t1179: f64, t161: f64, t165: f64, t177: f64, t3279: f64, t464: f64, t1450: f64, t1600: f64, t3031: f64, t458: f64, t3457: f64, t511: f64) -> (f64, f64, f64, f64, f64) {
    let t10134 = 28.0_f64 / 1215.0_f64 * t161 * t1179 * t165 * t177;
    let t10148 = t3279 * t464;
    let t10152 = t1450 * t1600;
    let t10178 = t458 * t3031;
    let t10185 = t511 * t3457;
    (t10134, t10148, t10152, t10178, t10185)
}
