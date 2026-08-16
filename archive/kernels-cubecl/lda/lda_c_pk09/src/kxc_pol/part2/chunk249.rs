//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 249/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk249<F: Float>(t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F) -> (F, F, F, F, F) {
    let t1081 = F::cast_from(0.6806222787477182_f64) * t666;
    let t1082 = F::cast_from(0.4537481858318121_f64) * t670;
    let t1086 = F::cast_from(0.04525483399593904_f64) * t612;
    let t1087 = F::cast_from(0.03016988933062603_f64) * t616;
    let t1091 = t1081 + t1082 + F::cast_from(0.6806222787477182_f64) * t676 + F::cast_from(0.6806222787477182_f64) * t681 - F::cast_from(0.6806222787477182_f64) * t687 + t1086 + t1087 + F::cast_from(0.04525483399593904_f64) * t626 + F::cast_from(0.04525483399593904_f64) * t636 - F::cast_from(0.04525483399593904_f64) * t653;
    (t1081, t1082, t1086, t1087, t1091)
}
