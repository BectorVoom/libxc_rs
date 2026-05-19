//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 797/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk797<F: Float>(t3317: F, t3335: F, t3342: F, t3871: F, t3873: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F) -> F {
    let t8013 = -F::cast_from(1.0416666666666667_f64) * t7801 - F::new(1.5625) * t7805 - F::new(1.5625) * t7809 - F::new(1.5625) * t7811 - F::new(1.5625) * t7814 - F::new(1.5625) * t7817 - F::new(1.5625) * t7834 - F::new(1.5625) * t3335 - F::cast_from(1.0416666666666667_f64) * t3342 + t3871 - t3873 + F::new(1.5625) * t3317;
    t8013
}
