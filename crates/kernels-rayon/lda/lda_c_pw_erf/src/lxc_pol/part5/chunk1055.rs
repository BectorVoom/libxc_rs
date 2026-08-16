//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1055/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1055(t156: f64, t7154: f64, t426: f64, t1652: f64, t2599: f64, t933: f64, t2611: f64, t325: f64, t415: f64, t7126: f64, t431: f64, t5594: f64, t7116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19551 = t156 * t7154;
    let t19552 = t426 * t19551;
    let t19571 = t1652 * t2599 * t933;
    let t19574 = t1652 * t2611 * t933;
    let t19577 = t415 * t7126 * t325;
    let t19580 = t431 * t7116 * t5594;
    (t19551, t19552, t19571, t19574, t19577, t19580)
}
