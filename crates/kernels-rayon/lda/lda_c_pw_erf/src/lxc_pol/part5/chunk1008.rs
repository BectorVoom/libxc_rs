//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1008/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1008(t558: f64, t6843: f64, t13080: f64, t571: f64, t6446: f64, t4804: f64, t6292: f64, t3859: f64, t519: f64, t6492: f64, t12695: f64, t6442: f64) -> (f64, f64, f64, f64, f64) {
    let t16121 = t6843 * t558;
    let t16127 = t571 * t13080 * t6446;
    let t16129 = t4804 * t6292;
    let t16134 = t519 * t3859 * t6492;
    let t16140 = t519 * t12695 * t6442;
    (t16121, t16127, t16129, t16134, t16140)
}
