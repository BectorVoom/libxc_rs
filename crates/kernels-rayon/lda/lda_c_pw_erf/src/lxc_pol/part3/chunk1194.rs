//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1194/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1194(t10643: f64, t10656: f64, t4589: f64, t544: f64, t14029: f64, t14033: f64, t14037: f64, t14040: f64, t14042: f64, t14045: f64, t14047: f64, t14050: f64, t14053: f64, t14054: f64) -> (f64, f64, f64, f64) {
    let t14055 = 32.0_f64 / 45.0_f64 * t10643;
    let t14056 = 32.0_f64 / 135.0_f64 * t10656;
    let t14058 = 4.0_f64 / 5.0_f64 * t4589 * t544;
    let t14059 = -t14029 - t14033 + t14037 - t14040 - t14042 - t14045 - t14047 - t14050 + t14053 + t14054 - t14055 - t14056 - t14058;
    (t14055, t14056, t14058, t14059)
}
