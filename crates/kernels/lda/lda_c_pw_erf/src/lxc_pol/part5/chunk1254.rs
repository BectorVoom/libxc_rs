//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1254/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1254<F: Float>(t331: F, t7405: F, t7409: F, t10195: F, t10202: F, t10225: F, t16287: F, t16292: F, t16297: F, t16325: F, t16327: F, t16338: F, t16345: F, t16365: F, t16370: F, t16372: F, t16374: F, t16382: F, t16397: F, t16399: F, t21847: F, t25: F, t589: F) -> F {
    let t22484 = t331 * t7405;
    let t22486 = t331 * t7409;
    let t22498 = F::new(0.08) * t16287 - F::cast_from(0.14396666666666666_f64) * t16292 + F::cast_from(0.03999074074074074_f64) * t16297 - F::cast_from(0.07198333333333333_f64) * t16325 + F::cast_from(0.023994444444444443_f64) * t16327 - F::cast_from(0.09597777777777777_f64) * t16338 + F::cast_from(0.09597777777777777_f64) * t16345 + t10195 + F::cast_from(0.019753086419753086_f64) * t10202 + F::cast_from(0.0044444444444444444_f64) * t22484 + F::cast_from(0.0019753086419753087_f64) * t22486 - F::cast_from(0.006666666666666667_f64) * t25 * t589 * t21847 + t10225 + F::cast_from(0.044444444444444446_f64) * t16365 - F::cast_from(0.022222222222222223_f64) * t16370 - F::cast_from(0.007407407407407408_f64) * t16372 + F::cast_from(0.035991666666666665_f64) * t16374 + F::cast_from(0.013333333333333334_f64) * t16382 - F::cast_from(0.047988888888888886_f64) * t16397 - F::cast_from(0.03199259259259259_f64) * t16399;
    t22498
}
