//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 699/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk699(t50: f64, t34: f64, t950: f64, t352: f64, t462: f64, t1789: f64, t1792: f64, t39: f64, t4367: f64, t52: f64, t951: f64, t954: f64, t4366: f64, t59: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t4370 = t950 * t34;
    let t4371 = t462 * t352;
    let t4381 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t4367 * t951 - 16.0_f64 / 9.0_f64 * t4370 * t4371 + 4.0_f64 / 9.0_f64 * t1789 * t954 - 8.0_f64 / 3.0_f64 * t52 * t462 + 8.0_f64 * t1792 * t39);
    let t4383 = (t4366 + t4381) * t59;
    (t4370, t4371, t4383)
}
