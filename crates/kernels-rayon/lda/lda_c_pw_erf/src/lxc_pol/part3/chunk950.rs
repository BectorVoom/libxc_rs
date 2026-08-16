//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 950/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk950(t1597: f64, t2881: f64, t2916: f64, t1: f64, t2872: f64, t482: f64, t485: f64, t1128: f64, t19: f64, t1098: f64, t2830: f64, t2833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10778 = t2881 * t2916 * t1597;
    let t10780 = t2872 * t1;
    let t10783 = 0.007901556131563792_f64 * t482 * t10780 * t485;
    let t10784 = t1128 * t19;
    let t10787 = 0.002972565416694299_f64 * t1098 * t10784 * t1597;
    let t10788 = t2830 * t485;
    let t10791 = 0.10359818039161417_f64 * t2833 * t485;
    (t10778, t10780, t10783, t10784, t10787, t10788, t10791)
}
