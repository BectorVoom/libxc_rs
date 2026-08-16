//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 242/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk242<F: Float>(t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F, t110: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t1029 = F::cast_from(4.59690841536205_f64) * t666;
    let t1030 = F::cast_from(3.0646056102413666_f64) * t670;
    let t1034 = F::cast_from(0.3056501876701794_f64) * t612;
    let t1035 = F::cast_from(0.2037667917801196_f64) * t616;
    let t1039 = t1029 + t1030 + F::cast_from(4.59690841536205_f64) * t676 + F::cast_from(4.59690841536205_f64) * t681 - F::cast_from(4.59690841536205_f64) * t687 + t1034 + t1035 + F::cast_from(0.3056501876701794_f64) * t626 + F::cast_from(0.3056501876701794_f64) * t636 - F::cast_from(0.3056501876701794_f64) * t653;
    let t1040 = t110 * t1039;
    let t1041 = t1040 * t89;
    (t1029, t1030, t1034, t1035, t1039, t1040, t1041)
}
