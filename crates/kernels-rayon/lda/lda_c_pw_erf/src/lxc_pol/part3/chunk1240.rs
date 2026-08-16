//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1240/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1240(t3296: f64, t9: f64, t155: f64, t1697: f64, t1870: f64, t5652: f64, t5515: f64, t925: f64, t2061: f64, t5518: f64, t325: f64, t415: f64, t5568: f64) -> (f64, f64, f64, f64, f64) {
    let t14674 = t9 * t3296;
    let t14679 = t155 * t1697;
    let t14681 = t1870 * t14679 * t5652;
    let t14683 = t5515 * t925;
    let t14684 = 1.9486833333333333_f64 * t14683;
    let t14685 = t5518 * t2061;
    let t14686 = 1.2991222222222223_f64 * t14685;
    let t14688 = t415 * t5568 * t325;
    (t14674, t14681, t14684, t14686, t14688)
}
