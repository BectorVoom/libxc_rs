//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 891/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk891(t1: f64, t3296: f64, t431: f64, t8916: f64, t119: f64, t1664: f64, t473: f64, t3210: f64, t155: f64, t3251: f64, t1657: f64, t2824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8917 = t431 * t3296 * t1 * t8916;
    let t8920 = t119 * t473 * t1664;
    let t8921 = t3210 * t8920;
    let t8924 = t119 * t155 * t3251;
    let t8925 = t1657 * t8924;
    let t8930 = t2824 * t1 * t119;
    (t8917, t8920, t8921, t8924, t8925, t8930)
}
