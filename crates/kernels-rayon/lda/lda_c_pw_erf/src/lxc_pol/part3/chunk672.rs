//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 672/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk672(t3440: f64, t3568: f64, t3721: f64, t3791: f64, t3858: f64, t3924: f64, t3989: f64, t4076: f64, t1210: f64, t168: f64, t671: f64, t1534: f64, t635: f64) -> (f64, f64, f64) {
    let t4079 = t3440 + t3568 + t3721 + t3791 + t3858 + t3924 + t3989 + t4076;
    let t4084 = t168 * t1210 * t671;
    let t4087 = t168 * t635 * t1534;
    (t4079, t4084, t4087)
}
