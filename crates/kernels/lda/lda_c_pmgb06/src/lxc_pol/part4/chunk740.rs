//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 740/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk740<F: Float>(t1385: F, t5294: F, t2010: F, t5244: F, t5247: F, t5250: F, t5252: F, t5256: F, t5259: F, t5263: F, t5266: F, t5270: F, t5275: F, t5279: F, t5284: F, t5286: F, t5289: F, t5293: F) -> (F, F, F) {
    let t5295 = t1385 * t5294;
    let t5297 = 4.0 / 45.0 * t2010 * t5295;
    let t5298 = -t5244 - t5247 - t5250 + t5252 + t5256 + t5259 + t5263 + t5266 - t5270 - t5275 - t5279 - t5284 - t5286 - t5289 - t5293 - t5297;
    (t5295, t5297, t5298)
}
