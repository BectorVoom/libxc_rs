//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 368/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk368<F: Float>(t1307: F, t1396: F, t1395: F, t1394: F, t113: F, t450: F) -> (F, F, F, F) {
    let t1397 = t1396 * t1307;
    let t1398 = t1395 * t1397;
    let t1399 = t1394 * t1398;
    let t1401 = t113 * t450;
    (t1397, t1398, t1399, t1401)
}
