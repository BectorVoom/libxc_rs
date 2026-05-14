//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 350/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk350<F: Float>(t489: F, t512: F, t161: F, t435: F, t459: F, t132: F, t134: F, t138: F, t1470: F, t350: F, t455: F, t139: F, t441: F) -> (F, F, F, F, F, F, F, F) {
    let t1504 = t489 * t512;
    let t1505 = t161 * t1504;
    let t1517 = t435 * t459;
    let t1518 = t132 * t1517;
    let t1521 = t138 * t1470 * t134;
    let t1522 = 0.002518888888888889 * t1521;
    let t1523 = t350 * t455;
    let t1525 = t139 * t441;
    (t1504, t1505, t1517, t1518, t1521, t1522, t1523, t1525)
}
