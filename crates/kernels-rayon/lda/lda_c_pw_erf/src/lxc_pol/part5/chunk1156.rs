//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1156/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1156(t1472: f64, t7713: f64, t16329: f64, t743: f64, t1319: f64, t571: f64, t34: f64, t6360: f64, t4758: f64, t348: f64, t7354: f64, t9777: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21230 = 8.0_f64 / 15.0_f64 * t1472 * t7713;
    let t21231 = t16329 * t743;
    let t21234 = 8.0_f64 / 15.0_f64 * t571 * t1319 * t21231;
    let t21235 = t6360 * t34;
    let t21238 = 16.0_f64 / 15.0_f64 * t571 * t4758 * t21235;
    let t21240 = t9777 * t7354 * t348;
    (t21230, t21231, t21234, t21235, t21238, t21240)
}
