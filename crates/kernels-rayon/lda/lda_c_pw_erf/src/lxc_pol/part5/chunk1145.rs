//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1145/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1145(t15685: f64, t6265: f64, t3863: f64, t571: f64, t7709: f64, t3854: f64, t7426: f64, t558: f64, t7836: f64, t1318: f64, t1319: f64, t352: f64) -> (f64, f64, f64, f64) {
    let t21093 = 8.0_f64 / 15.0_f64 * t15685 * t6265;
    let t21095 = t571 * t3863 * t7709;
    let t21096 = 8.0_f64 / 45.0_f64 * t21095;
    let t21098 = t571 * t3854 * t7426;
    let t21099 = 16.0_f64 / 45.0_f64 * t21098;
    let t21100 = t7836 * t558;
    let t21104 = 8.0_f64 / 45.0_f64 * t1318 * t1319 * t21100 * t352;
    (t21093, t21096, t21099, t21104)
}
