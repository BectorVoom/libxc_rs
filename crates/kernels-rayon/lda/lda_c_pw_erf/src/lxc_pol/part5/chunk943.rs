//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 943/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk943(t11397: f64, t4299: f64, t4415: f64, t2849: f64, t749: f64, t3160: f64, t3166: f64, t1765: f64, t2948: f64, t1077: f64, t4393: f64, t344: f64, t4405: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11398 = 1.898172889849454_f64 * t11397;
    let t11399 = t4415 * t4299;
    let t11401 = t2849 * t749;
    let t11403 = t3160 * t749;
    let t11404 = 240.0_f64 * t11403;
    let t11405 = t3166 * t749;
    let t11463 = t1765 * t2948;
    let t11465 = t4393 * t1077;
    let t11466 = 3.5089340384731225_f64 * t11465;
    let t11469 = t344 * t4405;
    (t11398, t11399, t11401, t11404, t11405, t11463, t11466, t11469)
}
