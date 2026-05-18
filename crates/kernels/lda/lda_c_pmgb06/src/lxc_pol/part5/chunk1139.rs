//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1139/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1139<F: Float>(t1423: F, t7651: F, t2493: F, t5220: F, t10134: F, t13292: F, t13295: F, t20663: F, t20666: F, t20667: F, t20668: F, t20670: F, t20671: F, t20673: F) -> (F, F, F) {
    let t20674 = t1423 * t7651;
    let t20675 = F::new(4.0) / F::new(45.0) * t20674;
    let t20676 = t5220 * t2493;
    let t20677 = F::new(4.0) / F::new(45.0) * t20676;
    let t20678 = -t20663 + t20666 - t13292 - t13295 - t20667 - t20668 + t20670 - t20671 - t10134 + t20673 - t20675 - t20677;
    (t20675, t20677, t20678)
}
