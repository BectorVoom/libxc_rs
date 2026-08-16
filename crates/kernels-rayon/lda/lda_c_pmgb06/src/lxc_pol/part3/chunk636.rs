//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 636/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk636(t3703: f64, t3709: f64, t967: f64, t696: f64, t971: f64, t980: f64, t1023: f64, t1026: f64, t109: f64, t138: f64, t1089: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t3711 = t3709 * t3703 * t967;
    let t3713 = 103.89515463408878_f64 * t696 * t3711;
    let t3714 = t971 * t980;
    let t3719 = 0.10685_f64 * t138 * t109 * t1023 * t1026;
    let t3720 = t1089 * t27;
    (t3711, t3713, t3714, t3719, t3720)
}
