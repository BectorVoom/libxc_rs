//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 595/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk595(t138: f64, t1706: f64, t1711: f64, t1712: f64, t1724: f64, t3327: f64, t3329: f64, t3332: f64, t3339: f64, t3340: f64, t3343: f64, t3363: f64, t444: f64, t450: f64) -> f64 {
    let t3365 = t3327 * t138 - 3.0_f64 * t1706 * t1724 + 6.0_f64 * t1711 * t3343 + 6.0_f64 * t3332 * t1712 - 3.0_f64 * t3329 * t450 - 6.0_f64 * t3339 * t3340 - t444 * t3363;
    t3365
}
