//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 807/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk807(t1856: f64, t431: f64, t325: f64, t1686: f64, t767: f64, t933: f64, t1833: f64, t415: f64, t1652: f64, t760: f64, t156: f64, t1844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5509 = t431 * t1856;
    let t5511 = 1.46904_f64 * t5509 * t325;
    let t5512 = t1686 * t767;
    let t5513 = t5512 * t933;
    let t5515 = t415 * t1833;
    let t5517 = 0.9743416666666667_f64 * t5515 * t325;
    let t5518 = t1652 * t760;
    let t5519 = t5518 * t933;
    let t5520 = 0.3247805555555556_f64 * t5519;
    let t5521 = t156 * t1844;
    (t5509, t5511, t5512, t5513, t5515, t5517, t5518, t5520, t5521)
}
