//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1058/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1058(t12387: f64, t12389: f64, t3965: f64, t11857: f64, t4488: f64, t3412: f64, t4483: f64, t12323: f64, t494: f64, t6710: f64, t1251: f64, t4489: f64) -> (f64, f64, f64, f64, f64) {
    let t12392 = 16.0_f64 / 5.0_f64 * t3965 * t12387 * t12389;
    let t12395 = 8.0_f64 / 5.0_f64 * t4488 * t12387 * t11857;
    let t12398 = 8.0_f64 / 15.0_f64 * t4488 * t4483 * t3412;
    let t12402 = 16.0_f64 / 15.0_f64 * t3965 * t6710 * t12323 * t494;
    let t12403 = t4489 * t1251;
    (t12392, t12395, t12398, t12402, t12403)
}
