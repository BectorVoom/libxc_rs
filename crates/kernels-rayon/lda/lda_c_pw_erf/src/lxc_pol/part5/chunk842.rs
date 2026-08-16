//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 842/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk842(t2415: f64, t833: f64, t1308: f64, t571: f64, t2396: f64, t4479: f64, t3965: f64, t2388: f64, t4475: f64, t3974: f64, t6791: f64, t2499: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7745 = t2415 * t833;
    let t7746 = t1308 * t7745;
    let t7748 = 8.0_f64 / 15.0_f64 * t571 * t7746;
    let t7749 = t4479 * t2396;
    let t7751 = 16.0_f64 / 15.0_f64 * t3965 * t7749;
    let t7752 = t4475 * t2388;
    let t7754 = 16.0_f64 / 15.0_f64 * t3974 * t7752;
    let t7755 = 8.0_f64 / 15.0_f64 * t6791;
    let t7757 = 2.0_f64 / 5.0_f64 * t795 * t2499;
    (t7745, t7746, t7748, t7749, t7751, t7752, t7754, t7755, t7757)
}
