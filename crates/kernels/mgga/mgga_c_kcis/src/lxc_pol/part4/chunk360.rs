//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 360/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk360<F: Float>(t498: F, t531: F, t833: F, t1370: F, t497: F) -> (F, F, F, F, F) {
    let t1371 = t498 * t531;
    let t1372 = t1371 * t833;
    let t1373 = t1370 * t1372;
    let t1376 = t497 * t497;
    let t1377 = 1.0 / t1376;
    (t1371, t1372, t1373, t1376, t1377)
}
