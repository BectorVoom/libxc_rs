//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 932/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk932<F: Float>(t19567: F, t19595: F, t5068: F, t5090: F, t6390: F, t1826: F, t2600: F, t1821: F, t5138: F, t17628: F, t1893: F, t5077: F, t2381: F, t822: F, t477: F, t6636: F) -> (F, F, F, F, F, F, F) {
    let t19596 = t19567 + t19595;
    let t19599 = 4.0 / 15.0 * t5068 * t5090 * t6390;
    let t19602 = 4.0 / 15.0 * t5068 * t2600 * t1826;
    let t19605 = 2.0 / 9.0 * t5138 * t2600 * t1821;
    let t19608 = 2.0 / 5.0 * t5077 * t17628 * t1893;
    let t19609 = t2381 * t822;
    let t19613 = 2.0 / 15.0 * t5077 * t6636 * t19609 * t477;
    (t19596, t19599, t19602, t19605, t19608, t19609, t19613)
}
