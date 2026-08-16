//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 441/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk441(t128: f64, t1686: f64, t933: f64, t1: f64, t436: f64, t431: f64) -> (f64, f64, f64, f64) {
    let t1687 = t1686 * t128;
    let t1689 = 0.16322666666666666_f64 * t1687 * t933;
    let t1690 = t436 * t1;
    let t1691 = t431 * t1690;
    (t1687, t1689, t1690, t1691)
}
