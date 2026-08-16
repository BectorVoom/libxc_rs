//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 823/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk823(t4730: f64, t6683: f64, t6686: f64, t6690: f64, t6697: f64, t6700: f64, t6703: f64, t6706: f64, t6708: f64, t2468: f64, t822: f64, t6193: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7545 = 4.0_f64 / 45.0_f64 * t4730;
    let t7547 = 16.0_f64 / 45.0_f64 * t6683;
    let t7548 = 16.0_f64 / 45.0_f64 * t6686;
    let t7549 = 16.0_f64 / 15.0_f64 * t6690;
    let t7550 = 8.0_f64 / 45.0_f64 * t6697;
    let t7551 = 8.0_f64 / 27.0_f64 * t6700;
    let t7552 = 8.0_f64 / 45.0_f64 * t6703;
    let t7553 = 8.0_f64 / 27.0_f64 * t6706;
    let t7554 = 16.0_f64 / 45.0_f64 * t6708;
    let t7556 = 4.0_f64 / 5.0_f64 * t822 * t2468;
    let t7557 = t6193 * t833;
    (t7545, t7547, t7548, t7549, t7550, t7551, t7552, t7553, t7554, t7556, t7557)
}
