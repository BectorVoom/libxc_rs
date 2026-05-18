//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 945/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk945<F: Float>(t11034: F, t11060: F, t257: F, t4481: F, t643: F, t4516: F, t638: F, t10997: F, t10999: F, t11000: F, t11002: F, t11003: F, t11007: F, t248: F, t285: F, t8482: F, t8519: F, t8520: F, t8526: F, t8534: F, t8541: F, t8543: F) -> (F, F) {
    let t11062 = (t11034 + t11060) * t257;
    let t11065 = t643 * t4481;
    let t11066 = F::new(24.0) * t11065;
    let t11067 = t638 * t4516;
    let t11069 = t248 * t11062 * t285 + t10997 - t10999 - t11000 - t11002 + t11003 + F::new(3.0) * t11007 - t11066 + F::new(12.0) * t11067 + t8482 - t8519 - F::new(360.0) * t8520 + t8526 + F::new(3.0) * t8534 - F::new(36.0) * t8541 + F::new(180.0) * t8543;
    (t11062, t11069)
}
