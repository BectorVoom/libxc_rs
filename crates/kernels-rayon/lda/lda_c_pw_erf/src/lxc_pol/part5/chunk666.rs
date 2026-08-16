//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 666/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk666(t1891: f64, t462: f64, t169: f64, t242: f64, t5466: f64, t2224: f64, t632: f64, t1143: f64, t875: f64, t1904: f64, t299: f64, t2220: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5750 = 0.2133002709687175_f64 * t462 * t1891;
    let t5760 = t169 * t5466 * t242;
    let t5768 = 0.06367133154935875_f64 * t169 * t2224 * t632;
    let t5770 = t169 * t875 * t1143;
    let t5772 = t299 * t1904;
    let t5775 = 0.10611888591559791_f64 * t169 * t5772 * t242;
    let t5777 = t169 * t2220 * t632;
    (t5750, t5760, t5768, t5770, t5772, t5775, t5777)
}
