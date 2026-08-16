//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 982/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk982(t344: f64, t5685: f64, t339: f64, t4384: f64, t1: f64, t1750: f64, t1755: f64, t2316: f64, t4299: f64, t4415: f64, t2849: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11387 = t344 * t5685;
    let t11388 = 24.0_f64 * t11387;
    let t11389 = t339 * t4384;
    let t11390 = 12.0_f64 * t11389;
    let t11391 = t344 * t4384;
    let t11392 = 12.0_f64 * t11391;
    let t11397 = t2316 * t1750 * t1 * t1755;
    let t11398 = 1.898172889849454_f64 * t11397;
    let t11399 = t4415 * t4299;
    let t11401 = t2849 * t749;
    (t11388, t11390, t11392, t11398, t11399, t11401)
}
