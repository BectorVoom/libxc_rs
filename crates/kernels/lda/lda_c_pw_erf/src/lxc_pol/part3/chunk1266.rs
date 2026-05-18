//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1266/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1266<F: Float>(t12199: F, t12203: F, t12205: F, t12207: F, t12211: F, t12215: F, t12219: F, t12223: F, t12227: F, t12229: F, t12234: F, t12239: F, t12241: F) -> F {
    let t15001 = -t12199 - t12203 + t12205 + t12207 + t12211 + t12215 + t12219 - t12223 - t12227 - t12229 - t12234 - t12239 - t12241;
    t15001
}
