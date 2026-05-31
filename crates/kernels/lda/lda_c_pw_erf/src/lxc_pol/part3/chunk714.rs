//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 714/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk714<F: Float>(t348: F, t4495: F, t4494: F, t4488: F, t1458: F, t529: F, t1245: F) -> (F, F, F, F, F) {
    let t4496 = t4495 * t348;
    let t4497 = t4494 * t4496;
    let t4499 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4488 * t4497;
    let t4500 = t1458 * t529;
    let t4501 = t4500 * t1245;
    (t4496, t4497, t4499, t4500, t4501)
}
