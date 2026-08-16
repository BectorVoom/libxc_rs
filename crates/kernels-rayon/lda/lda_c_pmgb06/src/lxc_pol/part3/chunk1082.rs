//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1082/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1082(t2965: f64, t439: f64, t5482: f64, t1444: f64, t5451: f64, t5454: f64, t3459: f64, t493: f64, t838: f64, t9908: f64, t2912: f64, t4856: f64) -> (f64, f64, f64, f64, f64) {
    let t12855 = 2.0_f64 / 15.0_f64 * t439 * t5482 * t2965;
    let t12857 = 2.0_f64 / 15.0_f64 * t1444 * t5451;
    let t12859 = 2.0_f64 / 3.0_f64 * t1444 * t5454;
    let t12863 = 2.0_f64 / 15.0_f64 * t493 * t9908 * t838 * t3459;
    let t12864 = t4856 * t2912;
    (t12855, t12857, t12859, t12863, t12864)
}
