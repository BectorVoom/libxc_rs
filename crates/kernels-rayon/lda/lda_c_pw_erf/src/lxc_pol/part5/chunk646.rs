//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 646/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk646(t1351: f64, t5165: f64, t2066: f64, t514: f64, t211: f64, t2071: f64, t4567: f64, t548: f64, t1397: f64, t2076: f64, t2099: f64, t185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5166 = t5165 * t1351;
    let t5170 = t514 * t2066;
    let t5172 = 8.0_f64 / 45.0_f64 * t211 * t5170;
    let t5175 = t4567 * t2071;
    let t5176 = t548 * t5175;
    let t5179 = 16.0_f64 / 45.0_f64 * t2076 * t1397;
    let t5184 = t514 * t2099;
    let t5186 = 8.0_f64 / 45.0_f64 * t185 * t5184;
    (t5166, t5170, t5172, t5175, t5176, t5179, t5184, t5186)
}
