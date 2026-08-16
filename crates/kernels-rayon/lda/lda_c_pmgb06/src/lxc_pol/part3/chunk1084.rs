//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1084/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1084(t1920: f64, t3223: f64, t2002: f64, t2949: f64, t2953: f64, t2962: f64, t5168: f64, t5264: f64, t1444: f64, t5494: f64, t2987: f64, t493: f64, t5486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12878 = t3223 * t1920;
    let t12879 = 2.0_f64 / 81.0_f64 * t12878;
    let t12881 = 2.0_f64 / 15.0_f64 * t2002 * t2949;
    let t12883 = t2002 * t2953 / 15.0_f64;
    let t12885 = t2002 * t2962 / 9.0_f64;
    let t12887 = 4.0_f64 / 9.0_f64 * t5168 * t5264;
    let t12889 = 2.0_f64 / 15.0_f64 * t1444 * t5494;
    let t12892 = t493 * t5486 * t2987 / 15.0_f64;
    (t12879, t12881, t12883, t12885, t12887, t12889, t12892)
}
