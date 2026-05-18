//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1276/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1276<F: Float>(t12982: F, t13008: F, t4937: F, t831: F, t13079: F, t9890: F, t9892: F, t9895: F, t9898: F, t16755: F, t16757: F, t16759: F, t16773: F, t16775: F, t16780: F, t16781: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16782 = F::new(8.0) / F::new(81.0) * t12982;
    let t16783 = F::new(16.0) / F::new(135.0) * t13008;
    let t16785 = t831 * t4937 / F::new(15.0);
    let t16786 = F::new(8.0) / F::new(135.0) * t13079;
    let t16787 = F::new(8.0) / F::new(405.0) * t9890;
    let t16788 = F::new(2.0) / F::new(135.0) * t9892;
    let t16789 = F::new(2.0) / F::new(135.0) * t9895;
    let t16790 = F::new(2.0) / F::new(135.0) * t9898;
    let t16791 = t16755 + t16757 + t16759 + t16773 + t16775 + t16780 + t16781 - t16782 + t16783 - t16785 + t16786 - t16787 + t16788 + t16789 - t16790;
    (t16782, t16783, t16785, t16786, t16787, t16788, t16789, t16790, t16791)
}
