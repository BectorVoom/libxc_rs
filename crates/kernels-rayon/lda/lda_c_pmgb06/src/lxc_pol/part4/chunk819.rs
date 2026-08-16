//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 819/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk819(t2262: f64, t707: f64, t2266: f64, t1773: f64, t909: f64, t3984: f64, t3987: f64, t3991: f64, t3995: f64, t3999: f64, t4005: f64, t4027: f64, t4234: f64, t5569: f64, t5573: f64, t5578: f64, t5580: f64, t5583: f64) -> (f64, f64, f64, f64) {
    let t5590 = 0.039914113367515366_f64 * t707 * t2262;
    let t5591 = t707 * t2266;
    let t5593 = t1773 * t909;
    let t5595 = 0.001355981270834723_f64 * t5569 + 0.19816831758676853_f64 * t5573 - t5578 - 0.0005811348303577384_f64 * t5580 - 6.0_f64 * t5583 * t4234 - 0.0002905674151788692_f64 * t3984 - 0.0011622696607154768_f64 * t3987 - t3991 + 0.002711962541669446_f64 * t3995 + t3999 - t4005 + t5590 + 0.039914113367515366_f64 * t5591 - 0.05321881782335382_f64 * t5593 - t4027;
    (t5590, t5591, t5593, t5595)
}
