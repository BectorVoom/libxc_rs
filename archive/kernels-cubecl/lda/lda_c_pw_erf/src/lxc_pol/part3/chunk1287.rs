//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1287/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1287<F: Float>(t12923: F, t12925: F, t12927: F, t12928: F, t12932: F, t12937: F, t12941: F, t12943: F, t12945: F, t12947: F, t12948: F, t12949: F, t12950: F, t12952: F) -> F {
    let t15054 = t12923 + t12925 + t12927 - t12928 + t12932 - t12937 + t12941 + t12943 + t12945 + t12947 + t12948 + t12949 + t12950 + t12952;
    t15054
}
