//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1163/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1163(t6568: f64, t795: f64, t2120: f64, t6592: f64, t16971: f64, t2505: f64, t6209: f64, t2104: f64, t7838: f64, t16935: f64, t16949: f64, t16952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21309 = 2.0_f64 / 5.0_f64 * t795 * t6568;
    let t21311 = 4.0_f64 / 5.0_f64 * t2120 * t6592;
    let t21313 = 4.0_f64 / 5.0_f64 * t16971 * t2505;
    let t21315 = 4.0_f64 / 5.0_f64 * t6209 * t6592;
    let t21317 = 4.0_f64 / 15.0_f64 * t2104 * t7838;
    let t21318 = 8.0_f64 / 27.0_f64 * t16935;
    let t21319 = 16.0_f64 / 45.0_f64 * t16949;
    let t21320 = 8.0_f64 / 45.0_f64 * t16952;
    (t21309, t21311, t21313, t21315, t21317, t21318, t21319, t21320)
}
