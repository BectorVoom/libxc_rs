//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 479/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk479<F: Float>(t3793: F, t1559: F, t1563: F, t1562: F, t597: F, t592: F, t3879: F, t600: F, t1341: F, t1347: F, t3918: F, t473: F, t3944: F, t187: F, t4114: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4318 = 0.22831111111111111111e-1 * t3793;
    let t4326 = t1559 * t1563;
    let t4329 = t1562 * t597;
    let t4330 = 1.0 / t4329;
    let t4331 = t592 * t4330;
    let t4338 = 0.68863333333333333333e0 * t3793;
    let t4345 = 0.17365833333333333333e0 * t3879;
    let t4354 = t1562 * t1562;
    let t4355 = 1.0 / t4354;
    let t4356 = t592 * t4355;
    let t4357 = t600 * t600;
    let t4358 = 1.0 / t4357;
    let t4363 = t1341 * t1347;
    let t4366 = t473 * t3918;
    let t4373 = t473 * t3944;
    let t4381 = t187 * t1341;
    let t4399 = 0.38691203703703703703e-3 * t4114;
    (t4318, t4326, t4330, t4331, t4338, t4345, t4354, t4355, t4356, t4357, t4358, t4363, t4366, t4373, t4381, t4399)
}
