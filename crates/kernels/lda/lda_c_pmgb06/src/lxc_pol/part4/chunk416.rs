//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 416/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk416<F: Float>(t1477: F, t1576: F, t1481: F, t525: F, t1485: F, t103: F, t1474: F, t1479: F, t1483: F, t1487: F, t1563: F, t1571: F, t1572: F) -> (F, F, F, F) {
    let t1577 = t1576 * t1477;
    let t1580 = t525 * t1481;
    let t1583 = t525 * t1485;
    let t1586 = t1563 + 0.023994444444444443 * t1474 - 0.023994444444444443 * t1479 + 0.07198333333333333 * t1483 - 0.035991666666666665 * t1487 + t1571 + 0.008888888888888889 * t1572 - 0.0022222222222222222 * t103 * t1577 + 0.013333333333333334 * t103 * t1580 - 0.006666666666666667 * t103 * t1583;
    (t1577, t1580, t1583, t1586)
}
