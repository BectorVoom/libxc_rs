//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 792/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk792<F: Float>(t5315: F, t549: F, t1466: F, t1318: F, t1401: F, t2065: F, t593: F, t571: F, t5259: F, t5263: F, t5268: F, t5274: F, t5278: F, t5281: F, t5284: F, t5288: F, t5294: F, t5298: F, t5301: F, t5304: F, t5309: F, t5312: F, t5314: F) -> (F, F, F, F, F, F, F, F) {
    let t5316 = t5315 * t549;
    let t5317 = t1466 * t5316;
    let t5319 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t5317;
    let t5320 = t1401 * t2065;
    let t5321 = t5320 * t593;
    let t5322 = t1466 * t5321;
    let t5324 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t571 * t5322;
    let t5325 = t5259 - t5263 + t5268 + t5274 + t5278 + t5281 + t5284 + t5288 + t5294 + t5298 + t5301 - t5304 + t5309 + t5312 - t5314 - t5319 + t5324;
    (t5316, t5317, t5319, t5320, t5321, t5322, t5324, t5325)
}
