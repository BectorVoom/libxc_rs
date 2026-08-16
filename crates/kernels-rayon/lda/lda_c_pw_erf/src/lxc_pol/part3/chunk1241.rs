//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1241/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1241(t14688: f64, t1652: f64, t1833: f64, t933: f64, t102: f64, t120: f64, t14632: f64, t1870: f64, t1872: f64, t436: f64, t473: f64, t5639: f64, t5643: f64) -> (f64, f64, f64, f64, f64) {
    let t14689 = 1.4615125_f64 * t14688;
    let t14691 = t1652 * t1833 * t933;
    let t14692 = 0.9743416666666667_f64 * t14691;
    let t14695 = 2.923025_f64 * t102 * t120 * t14632;
    let t14698 = t1870 * t473 * t436 * t1872;
    let t14701 = t1870 * t5639 * t5643;
    (t14689, t14692, t14695, t14698, t14701)
}
