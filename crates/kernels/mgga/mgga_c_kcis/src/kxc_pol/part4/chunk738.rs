//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 738/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk738<F: Float>(t1572: F, t4350: F, t1562: F, t592: F, t600: F, t4332: F, t1341: F, t1347: F, t3918: F, t473: F, t1356: F, t3919: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4351 = t4350 * t1572;
    let t4354 = t1562 * t1562;
    let t4355 = F::new(1.0) / t4354;
    let t4356 = t592 * t4355;
    let t4357 = t600 * t600;
    let t4358 = F::new(1.0) / t4357;
    let t4359 = t4332 * t4358;
    let t4363 = t1341 * t1347;
    let t4366 = t473 * t3918;
    let t4367 = t3919 * t1356;
    (t4351, t4354, t4355, t4356, t4357, t4358, t4359, t4363, t4366, t4367)
}
