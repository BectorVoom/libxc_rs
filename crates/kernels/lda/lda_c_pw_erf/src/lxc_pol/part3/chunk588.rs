//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 588/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk588<F: Float>(t3276: F, t3213: F, t3217: F, t3220: F, t3224: F, t3228: F, t3231: F, t3253: F, t3260: F, t3264: F, t3269: F, t3271: F, t3275: F, t426: F) -> (F, F) {
    let t3277 = F::new(1.9486833333333333) * t3276;
    let t3278 = -F::new(8.81424) * t3213 - F::new(2.93808) * t3217 - F::new(3.0) / F::new(2.0) * t3220 - F::new(6.0) * t426 * t3224 - F::new(2.0) / F::new(3.0) * t3228 + t3231 / F::new(2.0) - t426 * t3253 / F::new(2.0) - F::new(1.46904) * t3260 + F::new(2.20356) * t3264 + t3269 + t3271 - t3275 - t3277;
    (t3277, t3278)
}
