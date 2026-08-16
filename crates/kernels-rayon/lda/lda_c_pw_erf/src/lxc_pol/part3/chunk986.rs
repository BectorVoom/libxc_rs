//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 986/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk986(t8419: f64, t344: f64, t4405: f64, t1064: f64, t1799: f64, t390: f64, t40: f64, t4383: f64, t8438: f64, t169: f64, t301: f64, t5718: f64, t717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11468 = 3.0_f64 * t8419;
    let t11469 = t344 * t4405;
    let t11470 = 12.0_f64 * t11469;
    let t11471 = t1064 * t1799;
    let t11472 = 60.0_f64 * t11471;
    let t11474 = t40 * t4383 * t390;
    let t11475 = 3.0_f64 * t11474;
    let t11476 = 10.526802115419367_f64 * t8438;
    let t11482 = t169 * t717 * t5718 * t301;
    (t11468, t11470, t11472, t11475, t11476, t11482)
}
