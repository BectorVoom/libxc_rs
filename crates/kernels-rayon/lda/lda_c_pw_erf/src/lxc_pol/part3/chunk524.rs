//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 524/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk524(t1901: f64, t242: f64, t632: f64, t781: f64, t168: f64, t635: f64, t861: f64, t1904: f64, t247: f64, t251: f64, t652: f64, t850: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2240 = t1901 * t242;
    let t2244 = t781 * t632;
    let t2249 = t168 * t635 * t861;
    let t2252 = t1904 * t247;
    let t2253 = t2252 * t251;
    let t2256 = t850 * t652;
    (t2240, t2244, t2249, t2252, t2253, t2256)
}
