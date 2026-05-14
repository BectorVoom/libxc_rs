//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 925/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk925<F: Float>(t10705: F, t2382: F, t4699: F, t6835: F, t15993: F, t10572: F, t10574: F, t10576: F, t15991: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F, t2378: F, t2877: F) -> (F, F, F, F) {
    let t16361 = 1.0 * t10705 * t2382;
    let t16363 = 2.0 * t4699 * t6835;
    let t16379 = 0.39862222222222222222e0 * t15993;
    let t16386 = 0.66437037037037037038e-1 * t10572 - 0.19931111111111111111e0 * t10574 + 0.99655555555555555557e-1 * t10576 + 0.13287407407407407408e0 * t15991 - t16379 - 0.33218518518518518518e0 * t16001 + 0.11958666666666666667e1 * t16006 + 0.79724444444444444445e0 * t16011 - 0.19931111111111111111e0 * t16015 - 0.17938e1 * t16019 - 0.23917333333333333334e1 * t16024;
    let t16389 = t2877 * t2378;
    (t16361, t16363, t16386, t16389)
}
