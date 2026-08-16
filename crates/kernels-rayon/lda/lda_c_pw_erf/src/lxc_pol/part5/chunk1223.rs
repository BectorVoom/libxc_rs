//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1223/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1223(t2146: f64, t6362: f64, t6367: f64, t4763: f64, t6371: f64, t1325: f64, t2328: f64, t5289: f64, t542: f64, t806: f64, t11983: f64, t1318: f64, t2466: f64, t593: f64, t811: f64) -> (f64, f64, f64, f64, f64) {
    let t22084 = 8.0_f64 / 15.0_f64 * t2146 * t6362;
    let t22086 = 4.0_f64 / 9.0_f64 * t2146 * t6367;
    let t22088 = 8.0_f64 / 9.0_f64 * t4763 * t6371;
    let t22093 = 8.0_f64 / 5.0_f64 * t1325 * t5289 * t2328 * t806 * t542;
    let t22098 = 24.0_f64 / 5.0_f64 * t1318 * t11983 * t2466 * t811 * t593;
    (t22084, t22086, t22088, t22093, t22098)
}
