//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 443/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk443<F: Float>(t1738: F, t286: F, t1189: F, t1195: F, t125: F, t143: F, t1547: F, t1550: F, t1556: F, t1569: F, t1650: F, t1664: F, t1727: F, t1729: F, t1733: F, t1735: F, t279: F, t405: F, t453: F, t456: F) -> (F, F) {
    let t1740 = 0.05321881782335382 * t1738 * t286;
    let t1741 = 6.0 * t1729 * t143 * t1664 + t1650 * t125 + t1547 * t279 + t453 * t1550 - t453 * t1556 + 3.0 * t405 * t1569 + t1727 * t456 + 6.0 * t1733 * t1735 - t1189 + t1195 - t1740;
    (t1740, t1741)
}
