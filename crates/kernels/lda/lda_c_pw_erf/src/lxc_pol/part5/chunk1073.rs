//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1073/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1073<F: Float>(t15457: F, t15461: F, t11401: F, t11405: F, t11463: F, t11382: F, t11388: F, t11398: F, t11399: F, t11404: F, t8368: F, t8373: F, t8382: F, t8386: F, t8389: F, t8393: F, t8397: F, t8400: F) -> (F, F, F, F, F, F) {
    let t20074 = F::new(36.0) * t15457;
    let t20075 = F::new(12.0) * t15461;
    let t20077 = F::new(72.0) * t11401;
    let t20078 = F::new(360.0) * t11405;
    let t20079 = F::new(311.68360618876557) * t11463;
    let t20080 = -t20074 - t20075 - t11382 - t8368 - t8373 - t8382 + t8386 - t11388 - t8389 - t8393 + t8397 - t8400 - t11398 + F::new(9.49086444924727) * t11399 + t20077 - t11404 - t20078 + t20079;
    (t20074, t20075, t20077, t20078, t20079, t20080)
}
