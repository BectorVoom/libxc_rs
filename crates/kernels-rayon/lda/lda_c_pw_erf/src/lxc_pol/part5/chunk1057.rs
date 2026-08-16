//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1057/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1057(t2615: f64, t415: f64, t5594: f64, t19583: f64, t5607: f64, t2619: f64, t443: f64, t7166: f64, t1710: f64, t2630: f64, t1870: f64, t5639: f64, t7191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19645 = t415 * t2615 * t5594;
    let t19647 = t5607 * t19583;
    let t19650 = t415 * t2619 * t5594;
    let t19703 = t7166 * t443;
    let t19726 = t2630 * t1710;
    let t19739 = t1870 * t5639 * t7191;
    (t19645, t19647, t19650, t19703, t19726, t19739)
}
