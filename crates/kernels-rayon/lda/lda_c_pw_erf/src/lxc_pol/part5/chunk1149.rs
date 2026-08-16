//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1149/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1149(t21137: f64, t519: f64, t5256: f64, t4804: f64, t7702: f64, t3794: f64, t1472: f64, t7710: f64, t1308: f64, t571: f64, t6665: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t21140 = 8.0_f64 / 9.0_f64 * t519 * t5256 * t21137;
    let t21142 = 8.0_f64 / 9.0_f64 * t4804 * t7702;
    let t21144 = 8.0_f64 / 9.0_f64 * t3794 * t7702;
    let t21146 = 4.0_f64 / 15.0_f64 * t1472 * t7710;
    let t21150 = 4.0_f64 / 15.0_f64 * t571 * t1308 * t6665 * t833;
    (t21140, t21142, t21144, t21146, t21150)
}
