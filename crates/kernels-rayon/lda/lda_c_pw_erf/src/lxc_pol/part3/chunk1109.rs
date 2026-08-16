//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1109/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1109(t4507: f64, t811: f64, t3868: f64, t3974: f64, t2104: f64, t4571: f64, t10557: f64, t197: f64, t4610: f64, t519: f64, t11808: f64, t5250: f64) -> (f64, f64, f64, f64) {
    let t12968 = t4507 * t811;
    let t12971 = 16.0_f64 / 15.0_f64 * t3974 * t12968 * t3868;
    let t12974 = t2104 * t4571;
    let t12975 = 8.0_f64 / 45.0_f64 * t12974;
    let t12976 = t10557 * t197;
    let t12978 = t519 * t12976 * t4610;
    let t12979 = 64.0_f64 / 81.0_f64 * t12978;
    let t12982 = 128.0_f64 / 27.0_f64 * t519 * t5250 * t11808;
    (t12971, t12975, t12979, t12982)
}
