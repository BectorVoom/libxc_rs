//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1160/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1160(t5105: f64, t831: f64, t11750: f64, t11757: f64, t11762: f64, t11765: f64, t1596: f64, t2592: f64, t1: f64, t851: f64, t13672: f64, t529: f64, t6559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15256 = t831 * t5105;
    let t15257 = 4.0_f64 / 45.0_f64 * t15256;
    let t15258 = 4.0_f64 / 45.0_f64 * t11750;
    let t15259 = 4.0_f64 / 135.0_f64 * t11757;
    let t15260 = 4.0_f64 / 45.0_f64 * t11762;
    let t15261 = 2.0_f64 / 45.0_f64 * t11765;
    let t15263 = t2592 * t1596 / 15.0_f64;
    let t15264 = t1 * t851;
    let t15268 = 16.0_f64 / 45.0_f64 * t13672 * t6559 * t15264 * t529;
    (t15257, t15258, t15259, t15260, t15261, t15263, t15264, t15268)
}
