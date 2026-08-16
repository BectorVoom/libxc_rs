//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1138/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1138(t16069: f64, t16072: f64, t16075: f64, t1318: f64, t3854: f64, t7679: f64, t3863: f64, t571: f64, t7745: f64, t3802: f64, t519: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21014 = 8.0_f64 / 27.0_f64 * t16069;
    let t21015 = 64.0_f64 / 81.0_f64 * t16072;
    let t21016 = 32.0_f64 / 27.0_f64 * t16075;
    let t21018 = t1318 * t3854 * t7679;
    let t21019 = 32.0_f64 / 45.0_f64 * t21018;
    let t21021 = t571 * t3863 * t7745;
    let t21022 = 16.0_f64 / 45.0_f64 * t21021;
    let t21024 = t519 * t3802 * t7741;
    (t21014, t21015, t21016, t21019, t21022, t21024)
}
