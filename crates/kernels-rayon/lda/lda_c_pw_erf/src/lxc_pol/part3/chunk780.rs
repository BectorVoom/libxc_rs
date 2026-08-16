//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 780/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk780(t4029: f64, t4031: f64, t4033: f64, t2099: f64, t514: f64, t185: f64, t1394: f64, t795: f64, t2104: f64, t2137: f64, t1284: f64, t1298: f64, t2127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5181 = 8.0_f64 / 135.0_f64 * t4029;
    let t5182 = 4.0_f64 / 45.0_f64 * t4031;
    let t5183 = 8.0_f64 / 45.0_f64 * t4033;
    let t5184 = t514 * t2099;
    let t5186 = 8.0_f64 / 45.0_f64 * t185 * t5184;
    let t5188 = 4.0_f64 / 15.0_f64 * t795 * t1394;
    let t5190 = 16.0_f64 / 45.0_f64 * t2104 * t2137;
    let t5192 = 16.0_f64 / 45.0_f64 * t1284 * t2137;
    let t5194 = 16.0_f64 / 45.0_f64 * t1298 * t2127;
    (t5181, t5182, t5183, t5184, t5186, t5188, t5190, t5192, t5194)
}
