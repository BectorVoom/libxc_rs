//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 966/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk966(t13419: f64, t4906: f64, t529: f64, t1124: f64, t1458: f64, t197: f64, t4900: f64, t581: f64, t1484: f64, t219: f64, t2146: f64, t3763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13420 = 16.0_f64 / 135.0_f64 * t13419;
    let t13432 = t4906 * t529;
    let t13440 = t1124 * t1458 * t197;
    let t13444 = t4900 * t581;
    let t13455 = t1124 * t1484 * t219;
    let t13478 = t2146 * t3763;
    (t13420, t13432, t13440, t13444, t13455, t13478)
}
