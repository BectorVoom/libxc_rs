//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1005/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1005(t1621: f64, t2660: f64, t529: f64, t6590: f64, t1325: f64, t3787: f64, t6998: f64, t515: f64, t6631: f64, t2076: f64, t4571: f64, t2171: f64, t4834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15971 = t2660 * t1621;
    let t15975 = t529 * t6590;
    let t15983 = t1325 * t3787 * t6998;
    let t16016 = t6631 * t515;
    let t16024 = t2076 * t4571;
    let t16036 = t2171 * t4834;
    (t15971, t15975, t15983, t16016, t16024, t16036)
}
