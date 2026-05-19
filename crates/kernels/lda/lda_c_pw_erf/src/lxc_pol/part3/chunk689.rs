//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 689/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk689<F: Float>(t1953: F, t2061: F, t7: F, t226: F, t231: F, t4046: F, t4054: F, t4056: F, t4058: F, t4061: F, t4065: F, t4069: F, t4071: F, t4075: F, t4215: F, t4217: F, t4218: F, t4220: F, t4222: F, t4225: F, t4227: F) -> (F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t4231 = F::cast_from(1.2833333333333334_f64) * t1953 - F::new(20.0) / F::new(27.0) * t2061;
    let t4232 = t4231 * pi;
    let t4233 = t4232 * t7;
    let t4235 = F::new(4.0) / F::new(3.0) * t226 * t4233;
    let t4236 = t4046 + t4054 + t4056 + t4058 + t4061 + t4065 + t4069 + t4215 + t4217 + F::new(4.0) * t4218 + F::new(8.0) * t4220 + F::new(4.0) / F::new(3.0) * t4222 * t231 + F::new(4.0) * t4225 + F::new(4.0) * t4227 + t4235 - t4071 + t4075;
    (t4231, t4232, t4233, t4235, t4236)
}
