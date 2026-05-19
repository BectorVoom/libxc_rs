//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 348/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk348<F: Float>(t1271: F, t64: F, t955: F, t27: F, t365: F, t370: F) -> (F, F, F) {
    let t1272 = t1271 * t64;
    let t1274 = F::cast_from(0.16322666666666666_f64) * t1272 * t955;
    let t1276 = t365 * t370 * t27;
    (t1272, t1274, t1276)
}
