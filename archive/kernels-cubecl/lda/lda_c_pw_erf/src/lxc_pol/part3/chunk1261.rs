//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1261/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1261<F: Float>(t12039: F, t12040: F, t12041: F, t12042: F, t12043: F, t12045: F, t12047: F, t12049: F, t12051: F, t12053: F, t12055: F, t12059: F, t12063: F) -> F {
    let t14987 = t12039 + t12040 + t12041 + t12042 + t12043 - t12045 - t12047 - t12049 + t12051 - t12053 - t12055 + t12059 + t12063;
    t14987
}
