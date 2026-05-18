//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1268/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1268<F: Float>(t12277: F, t12279: F, t12281: F, t12285: F, t12292: F, t12294: F, t12296: F, t12298: F, t12301: F, t12305: F, t12308: F, t12310: F, t12312: F) -> F {
    let t15005 = -t12277 - t12279 + t12281 - t12285 + t12292 + t12294 - t12296 + t12298 - t12301 - t12305 + t12308 + t12310 - t12312;
    t15005
}
