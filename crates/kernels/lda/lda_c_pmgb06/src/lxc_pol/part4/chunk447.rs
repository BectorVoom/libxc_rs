//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 447/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk447<F: Float>(t1730: F, t206: F, t1562: F, t1590: F, t1598: F, t1606: F, t1633: F, t1635: F, t1638: F, t1643: F, t1708: F, t1712: F, t1727: F, t224: F) -> (F, F) {
    let t1732 = F::new(0.033245444444444446) * t206 * t1730;
    let t1733 = -t1562 - F::new(4.0) / F::new(45.0) * t1708 + t1712 - t1727 * t224 / F::new(15.0) - t1590 + t1598 + t1606 - t1633 - t1635 - t1638 - t1643 + t1732;
    (t1732, t1733)
}
