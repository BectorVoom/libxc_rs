//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 649/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk649(t1328: f64, t3859: f64, t1325: f64, t1475: f64, t581: f64) -> (f64, f64, f64, f64) {
    let t3860 = t3859 * t1328;
    let t3861 = t1325 * t3860;
    let t3862 = 32.0_f64 / 45.0_f64 * t3861;
    let t3863 = t1475 * t581;
    (t3860, t3861, t3862, t3863)
}
