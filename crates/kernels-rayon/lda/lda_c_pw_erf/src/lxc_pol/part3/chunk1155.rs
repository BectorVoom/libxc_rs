//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1155/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1155(t1184: f64, t2177: f64, t519: f64, t521: f64, t1321: f64, t2065: f64, t3974: f64, t3975: f64, t3424: f64, t5151: f64, t3420: f64, t13384: f64, t3429: f64) -> (f64, f64, f64, f64, f64) {
    let t13523 = t519 * t1184 * t521 * t2177;
    let t13524 = 128.0_f64 / 135.0_f64 * t13523;
    let t13528 = 16.0_f64 / 15.0_f64 * t3974 * t3975 * t2065 * t1321;
    let t13531 = 8.0_f64 / 15.0_f64 * t3974 * t5151 * t3424;
    let t13534 = 8.0_f64 / 15.0_f64 * t3974 * t5151 * t3420;
    let t13537 = 8.0_f64 / 9.0_f64 * t3974 * t13384 * t3429;
    (t13524, t13528, t13531, t13534, t13537)
}
