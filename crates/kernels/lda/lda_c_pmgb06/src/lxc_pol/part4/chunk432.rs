//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 432/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk432<F: Float>(t1696: F, t83: F, t208: F, t213: F, t580: F, t97: F, t588: F, t1384: F, t1389: F, t1391: F, t1394: F, t1676: F, t1679: F, t1682: F, t1684: F, t1689: F, t1692: F) -> (F, F, F, F, F, F) {
    let t1697 = t83 * t1696;
    let t1698 = t1697 * t208;
    let t1700 = t1698 * t213 / 3.0;
    let t1701 = t580 * t97;
    let t1703 = 0.12155555555555556 * t1701 * t588;
    let t1704 = 4.0 / 9.0 * t1676 + t1679 - t1682 + t1684 * t213 / 3.0 + 2.0 / 3.0 * t1689 + 0.12155555555555556 * t1692 + t1700 + t1703 - t1384 - t1389 - t1391 - t1394;
    (t1697, t1698, t1700, t1701, t1703, t1704)
}
