//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 901/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk901<F: Float>(t1494: F, t21971: F, t572: F, t571: F, t22411: F, t22413: F, t22415: F, t22417: F, t22420: F, t22423: F, t22425: F, t22428: F, t22431: F, t22433: F, t17412: F, t5919: F) -> (F, F, F) {
    let t22435 = t1494 * t21971;
    let t22436 = t572 * t22435;
    let t22437 = t571 * t22436;
    let t22439 = -t22411 / 72.0 + t22413 / 96.0 - t22415 / 128.0 - t22417 / 12.0 + 11.0 / 27.0 * t22420 - 19.0 / 108.0 * t22423 + t22425 / 128.0 + 19.0 / 144.0 * t22428 - t22431 / 64.0 - t22433 / 72.0 + t22437 / 24.0;
    let t22442 = t17412 * t5919;
    (t22437, t22439, t22442)
}
