//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1113/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1113(t3393: f64, t3965: f64, t4483: f64, t3818: f64, t12109: f64, t3398: f64, t10011: f64, t4480: f64, t108: f64, t2113: f64, t267: f64, t10015: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13025 = 8.0_f64 / 15.0_f64 * t3965 * t4483 * t3393;
    let t13028 = 8.0_f64 / 15.0_f64 * t3965 * t4483 * t3818;
    let t13031 = 8.0_f64 / 9.0_f64 * t3965 * t12109 * t3398;
    let t13032 = t10011 * t4480;
    let t13033 = 32.0_f64 / 45.0_f64 * t13032;
    let t13035 = t2113 * t108 * t267;
    let t13037 = 16.0_f64 / 15.0_f64 * t13035 * t4480;
    let t13039 = 16.0_f64 / 15.0_f64 * t10015 * t4480;
    (t13025, t13028, t13031, t13033, t13037, t13039)
}
