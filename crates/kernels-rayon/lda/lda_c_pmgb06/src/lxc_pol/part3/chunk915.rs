//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 915/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk915(t4087: f64, t588: f64, t97: f64, t1680: f64, t1684: f64, t1688: f64, t1691: f64, t4119: f64, t1698: f64, t1701: f64, t208: f64, t584: f64, t586: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10346 = t4087 * t97 * t588;
    let t10348 = t1684 * t1680;
    let t10350 = t1688 * t1680;
    let t10353 = t1691 * t4119;
    let t10356 = 4.0_f64 / 9.0_f64 * t1698 * t1680;
    let t10358 = 0.05402469135802469_f64 * t1701 * t4119;
    let t10362 = 0.05402469135802469_f64 * t584 * t586 * t740 * t208;
    (t10346, t10348, t10350, t10353, t10356, t10358, t10362)
}
