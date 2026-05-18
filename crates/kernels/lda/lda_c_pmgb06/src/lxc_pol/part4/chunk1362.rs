//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1362/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1362<F: Float>(t1594: F, t2578: F, t2864: F, t439: F, t1420: F, t6788: F, t6775: F, t2002: F, t5233: F, t2497: F, t3223: F, t1380: F, t1831: F, t1981: F, t2088: F) -> (F, F, F, F, F, F) {
    let t17902 = F::new(2.0) / F::new(45.0) * t439 * t2864 * t2578 * t1594;
    let t17904 = F::new(4.0) / F::new(45.0) * t1420 * t6788;
    let t17906 = F::new(2.0) / F::new(45.0) * t1420 * t6775;
    let t17908 = F::new(4.0) / F::new(45.0) * t2002 * t5233;
    let t17909 = t3223 * t2497;
    let t17910 = F::new(4.0) / F::new(405.0) * t17909;
    let t17914 = F::new(8.0) / F::new(45.0) * t1981 * t1380 * t1831 * t2088;
    (t17902, t17904, t17906, t17908, t17910, t17914)
}
