//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1020/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1020(t19181: f64, t19204: f64, t1: f64, t1981: f64, t2871: f64, t6516: f64, t1420: f64, t7563: f64, t15256: f64, t2095: f64, t2563: f64, t1887: f64, t2606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19205 = t19181 + t19204;
    let t19209 = 4.0_f64 / 15.0_f64 * t1981 * t2871 * t6516 * t1;
    let t19211 = 2.0_f64 / 15.0_f64 * t1420 * t7563;
    let t19215 = 2.0_f64 / 15.0_f64 * t15256;
    let t19217 = t2563 * t2095 / 10.0_f64;
    let t19219 = t1887 * t2606 / 5.0_f64;
    (t19205, t19209, t19211, t19215, t19217, t19219)
}
