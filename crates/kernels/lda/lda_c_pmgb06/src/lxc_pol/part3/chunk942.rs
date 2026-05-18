//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 942/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk942<F: Float>(t8483: F, t8527: F, t8529: F, t8536: F, t8538: F, t248: F, t4515: F, t686: F, t1069: F, t395: F, t247: F, t332: F) -> (F, F, F, F, F, F, F, F) {
    let t10997 = F::new(480.0) * t8483;
    let t10999 = F::new(96.0) * t8527;
    let t11000 = F::new(36.0) * t8529;
    let t11002 = F::new(48.0) * t8536;
    let t11003 = F::new(12.0) * t8538;
    let t11007 = t248 * t4515 * t686;
    let t11013 = t395 * t1069;
    let t11021 = t247 * t332;
    (t10997, t10999, t11000, t11002, t11003, t11007, t11013, t11021)
}
