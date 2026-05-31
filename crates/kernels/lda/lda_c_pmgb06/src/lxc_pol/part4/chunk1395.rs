//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1395/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1395<F: Float>(t11944: F, t16077: F, t16083: F, t16087: F, t16090: F, t16092: F, t16094: F, t16095: F, t16099: F, t9408: F, t9410: F, t9412: F, t9417: F, t9418: F, t9422: F) -> F {
    let t18203 = -F::cast_from(0.13298177777777778_f64) * t11944 - t16077 - t16083 - t16087 + t16090 - t16092 - t16094 - t16095 - t16099 - t9408 + t9410 + t9412 - t9417 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9418 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t9422;
    t18203
}
