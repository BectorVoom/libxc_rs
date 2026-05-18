//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1316/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1316<F: Float>(t2979: F, t493: F, t6755: F, t1380: F, t529: F, t6827: F, t1586: F, t2549: F, t10099: F, t10109: F, t13291: F, t13294: F) -> (F, F, F, F, F, F, F) {
    let t17296 = F::new(2.0) / F::new(45.0) * t493 * t2979 * t6755;
    let t17300 = F::new(2.0) / F::new(45.0) * t493 * t1380 * t6827 * t529;
    let t17304 = t493 * t1380 * t2549 * t1586 / F::new(45.0);
    let t17305 = F::new(2.0) / F::new(243.0) * t10099;
    let t17306 = F::new(2.0) / F::new(405.0) * t10109;
    let t17307 = F::new(4.0) / F::new(135.0) * t13291;
    let t17308 = F::new(4.0) / F::new(135.0) * t13294;
    (t17296, t17300, t17304, t17305, t17306, t17307, t17308)
}
