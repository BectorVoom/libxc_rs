//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 349/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk349<F: Float>(t1245: F, t1276: F, t366: F, t947: F, t18: F, t369: F) -> (F, F, F) {
    let t1277 = t1276 * t1245;
    let t1280 = F::cast_from(0.3264533333333333_f64) * t366 * t947;
    let t1282 = F::cast_from(1.0_f64) / t369 / t18;
    (t1277, t1280, t1282)
}
