//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 942/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk942<F: Float>(t2115: F, t2730: F, t93: F, t10962: F, t11450: F, t11452: F, t11456: F, t11458: F, t11462: F, t7224: F, t7226: F, t7228: F, t7230: F, t7232: F, t1947: F, t2825: F) -> (F, F, F) {
    let t11633 = t2115 * t2730;
    let t11634 = t93 * t11633;
    let t11657 = -t7224 / 36.0 - t7226 / 18.0 + t7228 / 18.0 + t7230 / 18.0 + t7232 / 18.0 + 0.037002892246025966 * t11450 + 0.037002892246025966 * t11452 - 0.037002892246025966 * t11456 - 0.14975624337724558 * t11458 - 0.14975624337724558 * t11462 + 0.03412591035063918 * t10962;
    let t11672 = t2825 * t1947;
    (t11634, t11657, t11672)
}
