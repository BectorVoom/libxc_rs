//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 826/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk826<F: Float>(t3317: F, t3335: F, t3342: F, t3599: F, t3601: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F) -> F {
    let t8360 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7801 - F::cast_from(2.0_f64) * t7805 - F::cast_from(2.0_f64) * t7809 - F::cast_from(2.0_f64) * t7811 - F::cast_from(2.0_f64) * t7814 - F::cast_from(2.0_f64) * t7817 - F::cast_from(2.0_f64) * t7834 - F::cast_from(2.0_f64) * t3335 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3342 + t3599 - t3601 + F::cast_from(2.0_f64) * t3317;
    t8360
}
