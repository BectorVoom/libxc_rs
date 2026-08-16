//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1105/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1105<F: Float>(t12879: F, t20264: F, t20268: F, t20271: F, t20274: F, t20275: F, t20279: F, t20281: F, t20283: F, t20284: F, t20285: F, t20288: F) -> F {
    let t20289 = -t12879 - t20264 - t20268 - t20271 + t20274 + t20275 - t20279 - t20281 - t20283 - t20284 + t20285 + t20288;
    t20289
}
