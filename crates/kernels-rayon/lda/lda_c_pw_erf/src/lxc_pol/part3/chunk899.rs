//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 899/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk899(t1704: f64, t2765: f64, t440: f64, t1553: f64, t1724: f64, t1555: f64, t1734: f64, t2790: f64, t1552: f64, t137: f64, t142: f64, t2777: f64) -> (f64, f64, f64, f64) {
    let t9121 = t2765 * t440 * t1704;
    let t9126 = t1553 * t1724;
    let t9127 = t9126 * t1555;
    let t9130 = t2790 * t1734;
    let t9133 = t1552 * t1552;
    let t9134 = 1.0_f64 / t9133;
    let t9135 = t9134 * t137;
    let t9138 = t9135 * t142 * t2777 * t440;
    (t9121, t9127, t9130, t9138)
}
