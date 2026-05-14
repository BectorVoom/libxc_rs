//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 535/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk535<F: Float>(t187: F, t856: F, t1550: F, t1557: F, t1645: F, t1646: F, t2041: F, t2045: F, t2068: F, t2070: F, t2092: F, t2097: F, t2099: F, t2103: F, t2105: F, t2110: F, t2111: F, t2113: F) -> (F, F) {
    let t2356 = t856 * t187;
    let t2358 = -t1550 - t2041 - t2045 - t2068 - t2070 - t2092 - t2097 - t2099 - t2103 - t2105 - t2110 + t2111 - t1557 - t2113 + t1645 + 4.0 / 3.0 * t1646 + 4.0 / 3.0 * t2356;
    (t2356, t2358)
}
