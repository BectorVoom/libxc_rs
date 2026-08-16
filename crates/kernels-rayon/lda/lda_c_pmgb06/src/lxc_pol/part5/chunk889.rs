//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 889/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk889(t1680: f64, t1698: f64, t1701: f64, t4119: f64, t208: f64, t584: f64, t586: f64, t740: f64, t3260: f64, t464: f64, t3031: f64, t442: f64) -> (f64, f64, f64, f64, f64) {
    let t10356 = 4.0_f64 / 9.0_f64 * t1698 * t1680;
    let t10358 = 0.05402469135802469_f64 * t1701 * t4119;
    let t10362 = 0.05402469135802469_f64 * t584 * t586 * t740 * t208;
    let t10431 = t3260 * t464;
    let t10439 = t442 * t3031;
    (t10356, t10358, t10362, t10431, t10439)
}
