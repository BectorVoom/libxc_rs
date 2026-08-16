//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 975/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk975(t197: f64, t3892: f64, t473: f64, t10527: f64, t219: f64, t10605: f64, t1944: f64, t571: f64, t9408: f64, t10162: f64, t1325: f64, t2167: f64) -> (f64, f64, f64, f64, f64) {
    let t14205 = t473 * t3892 * t197;
    let t14240 = t10527 * t219;
    let t14255 = t571 * t10605 * t219 * t1944;
    let t14256 = 8.0_f64 / 81.0_f64 * t14255;
    let t14257 = t9408 * t219;
    let t14313 = t1325 * t10162 * t2167;
    (t14205, t14240, t14256, t14257, t14313)
}
