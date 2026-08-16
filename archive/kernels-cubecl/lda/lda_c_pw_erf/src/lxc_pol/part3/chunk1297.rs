//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1297/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1297<F: Float>(t13319: F, t13325: F, t13327: F, t13329: F, t13334: F, t13338: F, t13340: F, t13342: F, t13347: F, t13349: F, t13352: F, t13356: F, t13359: F) -> F {
    let t15080 = -t13319 - t13325 + t13327 - t13329 + t13334 + t13338 - t13340 - t13342 + t13347 + t13349 + t13352 - t13356 - t13359;
    t15080
}
