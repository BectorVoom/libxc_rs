//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 916/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk916(t1325: f64, t3398: f64, t5237: f64, t3883: f64, t529: f64, t3403: f64, t519: f64, t3416: f64, t3855: f64, t4072: f64, t518: f64, t1251: f64, t177: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9721 = t1325 * t5237 * t3398;
    let t9723 = t3883 * t529;
    let t9725 = t519 * t9723 * t3403;
    let t9737 = t3416 * t3855;
    let t9752 = t4072 * t518;
    let t9761 = t191 / t177 / t1251;
    (t9721, t9723, t9725, t9737, t9752, t9761)
}
