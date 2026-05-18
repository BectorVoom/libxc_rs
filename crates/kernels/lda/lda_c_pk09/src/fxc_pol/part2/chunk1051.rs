//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1051/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1051<F: Float>(t1897: F, t2758: F, t11101: F, t1852: F, t1800: F, t2912: F, t6933: F, t1918: F, t454: F, t11092: F, t1931: F, t10: F, t6700: F) -> (F, F, F, F, F) {
    let t11433 = t1897 * t2758;
    let t11436 = t1852 * t11101;
    let t11437 = t11436 * t1800;
    let t11439 = t2912 * t6933;
    let t11440 = t11439 * t1918;
    let t11441 = t454 * t11440;
    let t11444 = t1931 * t11092;
    let t11449 = t6700 * t10;
    (t11433, t11437, t11441, t11444, t11449)
}
