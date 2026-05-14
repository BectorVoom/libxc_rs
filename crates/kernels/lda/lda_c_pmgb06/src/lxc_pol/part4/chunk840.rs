//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 840/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk840<F: Float>(t477: F, t6637: F, t6636: F, t5077: F, t332: F, t5094: F, t5084: F, t5083: F, t2563: F, t513: F, t1848: F, t844: F, t1837: F, t831: F, t6612: F, t6615: F, t6618: F, t6620: F, t6623: F, t6625: F, t6628: F, t6632: F, t6635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6638 = t6637 * t477;
    let t6639 = t6636 * t6638;
    let t6641 = 4.0 / 45.0 * t5077 * t6639;
    let t6642 = t6637 * t332;
    let t6643 = t5094 * t6642;
    let t6645 = 4.0 / 45.0 * t5077 * t6643;
    let t6646 = t5084 * t6642;
    let t6648 = 2.0 / 27.0 * t5083 * t6646;
    let t6650 = t2563 * t513 / 30.0;
    let t6652 = t1848 * t844 / 15.0;
    let t6654 = t831 * t1837 / 15.0;
    let t6655 = -t6612 - t6615 - t6618 - t6620 - t6623 - t6625 + t6628 + t6632 - t6635 + t6641 + t6645 - t6648 + t6650 + t6652 + t6654;
    (t6638, t6639, t6641, t6642, t6643, t6645, t6646, t6648, t6650, t6652, t6654, t6655)
}
