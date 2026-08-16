//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 953/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk953(t142: f64, t450: f64, t2766: f64, t1089: f64, t1191: f64, t169: f64, t301: f64, t3365: f64, t405: f64, t1554: f64, t3327: f64, t1549: f64, t2786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10832 = t450 * t142;
    let t10833 = t10832 * t2766;
    let t10838 = t169 * t1191 * t1089 * t301;
    let t10843 = t405 * t3365;
    let t10847 = t1554 * t142 * t3327;
    let t10849 = t1549 * t2786;
    (t10832, t10833, t10838, t10843, t10847, t10849)
}
