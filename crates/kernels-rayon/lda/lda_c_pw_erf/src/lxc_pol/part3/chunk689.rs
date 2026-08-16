//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 689/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk689(t1953: f64, t2061: f64, t7: f64, t226: f64, t231: f64, t4046: f64, t4054: f64, t4056: f64, t4058: f64, t4061: f64, t4065: f64, t4069: f64, t4071: f64, t4075: f64, t4215: f64, t4217: f64, t4218: f64, t4220: f64, t4222: f64, t4225: f64, t4227: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t4231 = 1.2833333333333334_f64 * t1953 - 20.0_f64 / 27.0_f64 * t2061;
    let t4232 = t4231 * pi;
    let t4233 = t4232 * t7;
    let t4235 = 4.0_f64 / 3.0_f64 * t226 * t4233;
    let t4236 = t4046 + t4054 + t4056 + t4058 + t4061 + t4065 + t4069 + t4215 + t4217 + 4.0_f64 * t4218 + 8.0_f64 * t4220 + 4.0_f64 / 3.0_f64 * t4222 * t231 + 4.0_f64 * t4225 + 4.0_f64 * t4227 + t4235 - t4071 + t4075;
    (t4231, t4232, t4233, t4235, t4236)
}
