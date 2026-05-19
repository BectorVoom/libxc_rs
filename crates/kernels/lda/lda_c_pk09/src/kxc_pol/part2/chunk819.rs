//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 819/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk819<F: Float>(t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F) -> F {
    let t8276 = -F::cast_from(0.9074963716636242_f64) * t7795 + F::cast_from(0.9074963716636242_f64) * t7797 + F::cast_from(0.9074963716636242_f64) * t7799 - F::cast_from(0.03016988933062603_f64) * t7801 - F::cast_from(0.04525483399593904_f64) * t7805 - F::cast_from(0.04525483399593904_f64) * t7809 - F::cast_from(0.04525483399593904_f64) * t7811 - F::cast_from(0.04525483399593904_f64) * t7814 - F::cast_from(0.04525483399593904_f64) * t7817 - F::cast_from(0.04525483399593904_f64) * t7834 - F::cast_from(0.6806222787477182_f64) * t7838 + F::cast_from(0.6806222787477182_f64) * t7842 + F::cast_from(0.6806222787477182_f64) * t7846;
    t8276
}
