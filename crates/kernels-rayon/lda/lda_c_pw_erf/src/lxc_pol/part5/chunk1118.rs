//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1118/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1118(t20729: f64, t3965: f64, t5141: f64, t20777: f64, t3967: f64, t494: f64, t4488: f64, t542: f64, t6710: f64, t2329: f64, t784: f64, t348: f64) -> (f64, f64, f64, f64, f64) {
    let t20787 = 16.0_f64 / 15.0_f64 * t3965 * t5141 * t20729;
    let t20791 = 8.0_f64 / 15.0_f64 * t3965 * t3967 * t20777 * t494;
    let t20795 = 8.0_f64 / 15.0_f64 * t4488 * t6710 * t20777 * t542;
    let t20796 = t2329 * t784;
    let t20800 = 8.0_f64 / 15.0_f64 * t3965 * t3967 * t20796 * t542;
    let t20801 = t20796 * t348;
    (t20787, t20791, t20795, t20800, t20801)
}
