//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 826/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk826(t39: f64, t865: f64, t1891: f64, t462: f64, t169: f64, t242: f64, t5466: f64, t171: f64, t4713: f64, t2224: f64, t632: f64, t1143: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5745 = t39 * t865;
    let t5750 = 0.2133002709687175_f64 * t462 * t1891;
    let t5760 = t169 * t5466 * t242;
    let t5762 = t171 * t4713;
    let t5768 = 0.06367133154935875_f64 * t169 * t2224 * t632;
    let t5770 = t169 * t875 * t1143;
    (t5745, t5750, t5760, t5762, t5768, t5770)
}
