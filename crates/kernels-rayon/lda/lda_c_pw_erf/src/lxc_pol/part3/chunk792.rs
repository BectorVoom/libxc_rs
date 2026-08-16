//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 792/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk792(t5315: f64, t549: f64, t1466: f64, t1318: f64, t1401: f64, t2065: f64, t593: f64, t571: f64, t5259: f64, t5263: f64, t5268: f64, t5274: f64, t5278: f64, t5281: f64, t5284: f64, t5288: f64, t5294: f64, t5298: f64, t5301: f64, t5304: f64, t5309: f64, t5312: f64, t5314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5316 = t5315 * t549;
    let t5317 = t1466 * t5316;
    let t5319 = 8.0_f64 / 15.0_f64 * t1318 * t5317;
    let t5320 = t1401 * t2065;
    let t5321 = t5320 * t593;
    let t5322 = t1466 * t5321;
    let t5324 = 8.0_f64 / 15.0_f64 * t571 * t5322;
    let t5325 = t5259 - t5263 + t5268 + t5274 + t5278 + t5281 + t5284 + t5288 + t5294 + t5298 + t5301 - t5304 + t5309 + t5312 - t5314 - t5319 + t5324;
    (t5316, t5317, t5319, t5320, t5321, t5322, t5324, t5325)
}
