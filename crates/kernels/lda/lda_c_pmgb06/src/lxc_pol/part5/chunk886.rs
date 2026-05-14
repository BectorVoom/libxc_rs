//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 886/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk886<F: Float>(t1555: F, t2563: F, t1423: F, t6297: F, t5108: F, t851: F, t5118: F, t822: F, t2599: F, t3458: F, t2604: F, t3032: F, t486: F, t6851: F, t6259: F, t6255: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17563 = t2563 * t1555;
    let t17577 = t1423 * t6297;
    let t17598 = t5108 * t851;
    let t17617 = t5118 * t822;
    let t17621 = t3458 * t2599;
    let t17628 = t3032 * t2604;
    let t17651 = t486 * t6851;
    let t17666 = t1423 * t6259;
    let t17668 = t1423 * t6255;
    (t17563, t17577, t17598, t17617, t17621, t17628, t17651, t17666, t17668)
}
