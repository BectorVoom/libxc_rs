//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 747/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk747<F: Float>(t1507: F, t456: F, t3393: F, t4232: F, t238: F, t4239: F, t86: F, t4236: F, t4222: F, t1523: F, t318: F, t334: F, t565: F, t4106: F, t531: F, t4227: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12361 = t1507 * t456;
    let t12381 = t3393 * t4232;
    let t12390 = t86 * t238 * t4239;
    let t12392 = t3393 * t4236;
    let t12394 = t3393 * t4222;
    let t12397 = t86 * t318 * t1523;
    let t12401 = 0.11791604938271604938e-1 * t86 * t334 * t565;
    let t12417 = t4106 * t531;
    let t12427 = t3393 * t4227;
    (t12361, t12381, t12390, t12392, t12394, t12397, t12401, t12417, t12427)
}
