//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1121/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1121<F: Float>(t13079: F, t9890: F, t9892: F, t9895: F, t9898: F, t16755: F, t16757: F, t16759: F, t16773: F, t16775: F, t16780: F, t16781: F, t16782: F, t16783: F, t16785: F, t495: F, t6831: F) -> (F, F, F, F, F, F, F) {
    let t16786 = 8.0 / 135.0 * t13079;
    let t16787 = 8.0 / 405.0 * t9890;
    let t16788 = 2.0 / 135.0 * t9892;
    let t16789 = 2.0 / 135.0 * t9895;
    let t16790 = 2.0 / 135.0 * t9898;
    let t16791 = t16755 + t16757 + t16759 + t16773 + t16775 + t16780 + t16781 - t16782 + t16783 - t16785 + t16786 - t16787 + t16788 + t16789 - t16790;
    let t16794 = t495 * t6831;
    (t16786, t16787, t16788, t16789, t16790, t16791, t16794)
}
