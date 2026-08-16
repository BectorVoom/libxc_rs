//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1109/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1109(t3384: f64, t831: f64, t1636: f64, t1848: f64, t2880: f64, t4612: f64, t5211: f64, t1983: f64, t485: f64, t5210: f64, t5322: f64, t5499: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13191 = t831 * t3384 / 30.0_f64;
    let t13192 = t1848 * t1636;
    let t13193 = 2.0_f64 / 15.0_f64 * t13192;
    let t13194 = t831 * t2880;
    let t13195 = 2.0_f64 / 15.0_f64 * t13194;
    let t13196 = t5211 * t4612;
    let t13197 = 2.0_f64 / 9.0_f64 * t13196;
    let t13199 = t485 * t5210 * t1983;
    let t13200 = 2.0_f64 / 9.0_f64 * t13199;
    let t13201 = t5499 * t5322;
    (t13191, t13193, t13195, t13197, t13200, t13201)
}
