//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1160/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1160(t16876: f64, t6306: f64, t795: f64, t2123: f64, t2425: f64, t2076: f64, t6215: f64, t16912: f64, t16918: f64, t16922: f64, t2137: f64, t6601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21269 = 16.0_f64 / 27.0_f64 * t16876;
    let t21270 = t795 * t6306;
    let t21271 = 8.0_f64 / 15.0_f64 * t21270;
    let t21273 = t2425 * t2123;
    let t21274 = 4.0_f64 / 15.0_f64 * t21273;
    let t21275 = t2076 * t6215;
    let t21276 = 8.0_f64 / 15.0_f64 * t21275;
    let t21277 = 4.0_f64 / 15.0_f64 * t16912;
    let t21278 = 16.0_f64 / 15.0_f64 * t16918;
    let t21279 = 16.0_f64 / 15.0_f64 * t16922;
    let t21280 = t6601 * t2137;
    (t21269, t21271, t21274, t21276, t21277, t21278, t21279, t21280)
}
