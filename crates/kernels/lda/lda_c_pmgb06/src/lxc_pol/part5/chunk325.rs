//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 325/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk325<F: Float>(t1238: F, t39: F, t955: F, t27: F, t348: F, t56: F, t109: F, t342: F, t55: F, t349: F, t947: F, t409: F, t54: F) -> (F, F, F, F, F, F, F) {
    let t1239 = t1238 * t39;
    let t1241 = F::cast_from(0.3247805555555556_f64) * t1239 * t955;
    let t1243 = t348 * t56 * t27;
    let t1245 = t55 * t109 * t342;
    let t1246 = t1243 * t1245;
    let t1249 = F::cast_from(0.6495611111111111_f64) * t349 * t947;
    let t1259 = t54 * t55 * t409 * t56 / F::new(9.0);
    (t1239, t1241, t1243, t1245, t1246, t1249, t1259)
}
