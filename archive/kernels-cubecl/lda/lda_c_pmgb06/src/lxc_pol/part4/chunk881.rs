//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 881/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk881<F: Float>(t2012: F, t6275: F, t1420: F, t2477: F, t1444: F, t2470: F, t2469: F, t3238: F, t493: F, t2599: F, t3457: F, t529: F) -> (F, F, F, F, F, F, F) {
    let t6277 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6275 * t2012;
    let t6279 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1420 * t2477;
    let t6281 = t1444 * t2470 / F::cast_from(27.0_f64);
    let t6282 = t3238 * t2469;
    let t6284 = t493 * t6282 / F::cast_from(27.0_f64);
    let t6285 = t3457 * t2599;
    let t6286 = t6285 * t529;
    (t6277, t6279, t6281, t6282, t6284, t6285, t6286)
}
