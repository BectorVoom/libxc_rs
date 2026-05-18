//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 449/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk449<F: Float>(t707: F, t711: F, t715: F, t113: F, t1166: F, t301: F, t398: F, t413: F, t297: F, t1183: F, t83: F, t246: F, t33: F) -> (F, F, F, F, F, F, F, F) {
    let t1750 = t707 * t711;
    let t1753 = F::new(0.039914113367515366) * t707 * t715;
    let t1755 = t1166 * t113 * t301;
    let t1759 = t398 * t413 * t301;
    let t1760 = t297 * t1759;
    let t1763 = t83 * t1183 * t301;
    let t1765 = F::new(0.01197423401025461) * t297 * t1763;
    let t1767 = F::new(1.0) / t33 / t246;
    (t1750, t1753, t1755, t1759, t1760, t1763, t1765, t1767)
}
