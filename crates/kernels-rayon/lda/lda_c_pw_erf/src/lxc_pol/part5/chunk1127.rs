//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1127/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1127(t9246: f64, t15743: f64, t15750: f64, t2031: f64, t6198: f64, t1987: f64, t1992: f64, t10056: f64, t352: f64, t7365: f64, t4776: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20897 = 16.0_f64 / 405.0_f64 * t9246;
    let t20898 = 32.0_f64 / 45.0_f64 * t15743;
    let t20899 = 16.0_f64 / 27.0_f64 * t15750;
    let t20901 = 4.0_f64 / 15.0_f64 * t6198 * t2031;
    let t20903 = 8.0_f64 / 15.0_f64 * t6198 * t1987;
    let t20905 = 4.0_f64 / 9.0_f64 * t6198 * t1992;
    let t20907 = t10056 * t7365 * t352;
    let t20910 = 128.0_f64 / 27.0_f64 * t571 * t4776 * t20907;
    (t20897, t20898, t20899, t20901, t20903, t20905, t20907, t20910)
}
