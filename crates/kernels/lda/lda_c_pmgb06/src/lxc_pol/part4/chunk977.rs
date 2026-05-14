//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 977/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk977<F: Float>(t1944: F, t642: F, t1952: F, t12514: F, t1461: F, t5065: F, t5140: F, t1438: F, t154: F, t3098: F, t465: F, t495: F, t5072: F, t12535: F, t1435: F, t5075: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12939 = 64.0 * t1944 * t642;
    let t12960 = 64.0 * t1952 * t642;
    let t12981 = t5065 * t12514 * t1461;
    let t12982 = t12981 * t5140;
    let t12991 = t154 * t1438;
    let t12995 = t154 * t3098;
    let t13000 = t465 * t3098;
    let t13007 = t5065 * t12514 * t495;
    let t13008 = t13007 * t5072;
    let t13020 = t5075 * t12535 * t1435;
    (t12939, t12960, t12981, t12982, t12991, t12995, t13000, t13007, t13008, t13020)
}
