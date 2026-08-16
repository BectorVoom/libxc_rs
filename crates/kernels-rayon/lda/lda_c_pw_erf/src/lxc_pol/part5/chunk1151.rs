//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1151/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1151(t1308: f64, t2065: f64, t2419: f64, t571: f64, t1446: f64, t7706: f64, t2098: f64, t2429: f64, t3402: f64, t519: f64, t1325: f64, t1991: f64, t494: f64, t7639: f64) -> (f64, f64, f64, f64) {
    let t21159 = 4.0_f64 / 15.0_f64 * t571 * t1308 * t2419 * t2065;
    let t21161 = 4.0_f64 / 9.0_f64 * t1446 * t7706;
    let t21165 = 4.0_f64 / 9.0_f64 * t519 * t3402 * t2429 * t2098;
    let t21169 = 16.0_f64 / 9.0_f64 * t1325 * t1991 * t7639 * t494;
    (t21159, t21161, t21165, t21169)
}
