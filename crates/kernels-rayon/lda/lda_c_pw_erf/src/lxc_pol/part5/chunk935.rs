//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 935/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk935(t1210: f64, t645: f64, t646: f64, t1410: f64, t1419: f64, t1423: f64, t3936: f64, t10967: f64, t168: f64, t270: f64, t2782: f64, t671: f64) -> (f64, f64, f64, f64, f64) {
    let t11159 = 0.05402469135802469_f64 * t645 * t1210 * t646;
    let t11166 = t1419 * t1410;
    let t11168 = t1423 * t3936;
    let t11196 = 0.9079060239445599_f64 * t168 * t10967 * t270;
    let t11198 = t168 * t2782 * t671;
    (t11159, t11166, t11168, t11196, t11198)
}
