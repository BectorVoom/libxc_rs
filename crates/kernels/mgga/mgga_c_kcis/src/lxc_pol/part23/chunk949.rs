//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 949/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk949<F: Float>(t4298: F, t6002: F, t16609: F, t556: F, t572: F, t1533: F, t16721: F, t4261: F, t6027: F, t2042: F, t4273: F, t571: F) -> (F, F, F, F, F) {
    let t17484 = t6002 * t4298;
    let t17486 = t556 * t16609;
    let t17487 = t572 * t17486;
    let t17488 = t1533 * t17487;
    let t17490 = t4261 * t16721;
    let t17491 = t6027 * t17490;
    let t17493 = t2042 * t4273;
    let t17494 = t571 * t17493;
    (t17484, t17488, t17490, t17491, t17494)
}
