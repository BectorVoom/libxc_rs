//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1119/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1119(t20801: f64, t3965: f64, t5141: f64, t5147: f64, t2337: f64, t743: f64, t593: f64, t4506: f64, t4515: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20804 = 16.0_f64 / 15.0_f64 * t3965 * t5141 * t20801;
    let t20807 = 8.0_f64 / 9.0_f64 * t3965 * t5147 * t20801;
    let t20808 = t2337 * t743;
    let t20809 = t20808 * t593;
    let t20812 = 8.0_f64 / 15.0_f64 * t4506 * t4515 * t20809;
    let t20813 = t20808 * t352;
    (t20804, t20807, t20808, t20809, t20812, t20813)
}
