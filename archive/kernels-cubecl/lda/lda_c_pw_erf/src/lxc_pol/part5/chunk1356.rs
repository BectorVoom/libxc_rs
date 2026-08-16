//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1356/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1356<F: Float>(t23049: F, t23050: F, t23053: F, t23054: F, t23055: F, t23056: F, t23057: F, t23058: F, t23059: F, t23060: F, t23061: F, t23062: F, t23064: F) -> F {
    let t23334 = t23049 - t23050 - t23053 - t23054 + t23055 - t23056 + t23057 - t23058 - t23059 - t23060 - t23061 + t23062 - t23064;
    t23334
}
