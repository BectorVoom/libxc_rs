//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 643/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk643(t3785: f64, t667: f64, t273: f64, t3738: f64, t3703: f64, t3741: f64, t3709: f64, t967: f64, t409: f64, t675: f64, t109: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3800 = t3785 * t667;
    let t3803 = t273 * t3738;
    let t3804 = t3703 * t3741;
    let t3807 = t273 * t3709;
    let t3808 = t3703 * t967;
    let t3811 = t409 * t675;
    let t3818 = t109 * t963;
    (t3800, t3803, t3804, t3807, t3808, t3811, t3818)
}
