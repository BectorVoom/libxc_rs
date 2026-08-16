//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1342/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1342<F: Float>(t22246: F, t22249: F, t22252: F, t22254: F, t22256: F, t22257: F, t22263: F, t22266: F, t22268: F, t22272: F, t22276: F, t22280: F, t22284: F) -> F {
    let t23297 = -t22246 + t22249 - t22252 - t22254 - t22256 - t22257 + t22263 + t22266 - t22268 + t22272 - t22276 - t22280 + t22284;
    t23297
}
