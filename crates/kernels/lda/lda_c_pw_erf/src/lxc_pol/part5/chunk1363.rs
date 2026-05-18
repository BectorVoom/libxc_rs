//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1363/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1363<F: Float>(t11272: F, t11273: F, t11274: F, t11275: F, t11276: F, t11277: F, t11282: F, t11286: F, t15321: F, t15322: F, t15323: F, t3172: F, t3178: F, t3180: F, t3182: F, t3184: F, t6067: F, t6070: F, t6072: F, t7384: F) -> F {
    let t23372 = t11272 - t11273 + t11274 + t11275 - t11276 + t11277 + t15321 - F::new(24.0) * t6067 + F::new(3.0) * t6070 - t15322 - t11282 + F::new(6.0) * t6072 + t7384 + t3172 + t15323 + t11286 + t3178 + t3180 - t3182 - t3184;
    t23372
}
