//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1248/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1248<F: Float>(t12644: F, t12649: F, t1444: F, t6533: F, t531: F, t6688: F, t1641: F, t2563: F, t1588: F, t1592: F, t2582: F, t132: F, t137: F, t1594: F) -> (F, F, F, F, F, F, F) {
    let t16424 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t12644;
    let t16425 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t12649;
    let t16427 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1444 * t6533;
    let t16429 = t6688 * t531 / F::cast_from(15.0_f64);
    let t16431 = t2563 * t1641 / F::cast_from(15.0_f64);
    let t16433 = t2563 * t1588 / F::cast_from(30.0_f64);
    let t16434 = t2582 * t1592;
    let t16438 = t132 * t137 * t16434 * t1594 / F::cast_from(15.0_f64);
    (t16424, t16425, t16427, t16429, t16431, t16433, t16438)
}
