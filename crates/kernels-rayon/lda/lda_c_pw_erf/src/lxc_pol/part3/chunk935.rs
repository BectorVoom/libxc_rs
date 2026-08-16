//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 935/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk935(t1318: f64, t3837: f64, t3854: f64, t1620: f64, t598: f64, t226: f64, t4232: f64, t1159: f64, t603: f64, t1634: f64, t695: f64, t2070: f64, t493: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10403 = t1318 * t3854 * t3837;
    let t10409 = t598 * t1620;
    let t10412 = 16.0_f64 / 3.0_f64 * t226 * t4232;
    let t10414 = t1159 * t603;
    let t10417 = 0.004413481481481482_f64 * t695 * t1634;
    let t10419 = t493 * t2070 * t495;
    (t10403, t10409, t10412, t10414, t10417, t10419)
}
