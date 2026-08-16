//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1357/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1357(t13891: f64, t13893: f64, t13895: f64, t13905: f64, t13907: f64, t13909: f64, t13911: f64, t13913: f64, t13915: f64, t13917: f64, t13920: f64, t13922: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17842 = 16.0_f64 / 135.0_f64 * t13891;
    let t17843 = 16.0_f64 / 135.0_f64 * t13893;
    let t17844 = 16.0_f64 / 135.0_f64 * t13895;
    let t17845 = 8.0_f64 / 135.0_f64 * t13905;
    let t17846 = 16.0_f64 / 135.0_f64 * t13907;
    let t17847 = 8.0_f64 / 81.0_f64 * t13909;
    let t17848 = 8.0_f64 / 135.0_f64 * t13911;
    let t17849 = 16.0_f64 / 135.0_f64 * t13913;
    let t17850 = 8.0_f64 / 81.0_f64 * t13915;
    let t17851 = 16.0_f64 / 135.0_f64 * t13917;
    let t17852 = 32.0_f64 / 135.0_f64 * t13920;
    let t17853 = 16.0_f64 / 81.0_f64 * t13922;
    (t17842, t17843, t17844, t17845, t17846, t17847, t17848, t17849, t17850, t17851, t17852, t17853)
}
