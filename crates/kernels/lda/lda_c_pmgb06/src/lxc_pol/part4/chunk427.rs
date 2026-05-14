//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 427/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk427<F: Float>(t1639: F, t529: F, t166: F, t161: F, t187: F, t540: F, t534: F, t1553: F, t1557: F, t1562: F, t1590: F, t1598: F, t1606: F, t1633: F, t1635: F, t1638: F) -> (F, F, F, F, F, F) {
    let t1640 = t1639 * t529;
    let t1641 = t166 * t1640;
    let t1643 = t161 * t1641 / 15.0;
    let t1645 = 8.0 / 3.0 * t540 * t187;
    let t1646 = t534 * t187;
    let t1648 = t1553 - t1557 - t1562 - t1590 + t1598 + t1606 - t1633 - t1635 - t1638 - t1643 + t1645 + 8.0 / 3.0 * t1646;
    (t1640, t1641, t1643, t1645, t1646, t1648)
}
