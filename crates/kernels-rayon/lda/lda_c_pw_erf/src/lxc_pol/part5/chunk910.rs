//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 910/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk910(t2151: f64, t576: f64, t571: f64, t2070: f64, t548: f64, t550: f64, t1401: f64, t1475: f64, t3893: f64, t529: f64, t3883: f64, t1251: f64, t177: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9436 = t2151 * t576;
    let t9437 = t571 * t9436;
    let t9593 = t548 * t2070 * t550;
    let t9678 = t1475 * t1401;
    let t9700 = t3893 * t529;
    let t9723 = t3883 * t529;
    let t9761 = t191 / t177 / t1251;
    (t9437, t9593, t9678, t9700, t9723, t9761)
}
