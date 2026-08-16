//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 553/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk553(t171: f64, t2853: f64, t1113: f64, t169: f64, t632: f64, t1143: f64, t703: f64, t161: f64, t2872: f64, t1: f64, t1128: f64) -> (f64, f64, f64, f64, f64) {
    let t2898 = t171 * t2853;
    let t2903 = t169 * t1113 * t632;
    let t2906 = t169 * t703 * t1143;
    let t2908 = t2872 * t161;
    let t2910 = t1128 * t1;
    (t2898, t2903, t2906, t2908, t2910)
}
