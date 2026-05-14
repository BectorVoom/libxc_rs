//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1016/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1016<F: Float>(t4075: F, t6082: F, t13682: F, t2128: F, t4032: F, t1248: F, t13614: F, t2075: F, t20295: F, t20298: F, t13526: F, t13530: F, t13533: F, t13536: F, t13618: F, t20292: F, t20302: F, t20305: F, t20308: F, t20312: F, t20315: F, t20318: F, t20321: F, t20324: F, t20327: F) -> (F, F, F, F) {
    let t20361 = t6082 * t4075;
    let t20364 = t2128 * t13682;
    let t20365 = t20364 * t4032;
    let t20373 = t1248 * t13614 * t2075;
    let t20381 = 4.0 / 27.0 * t20295;
    let t20382 = 4.0 / 9.0 * t20298;
    let t20392 = -t13618 - 8.0 / 27.0 * t13526 + 2.0 / 27.0 * t13530 - 2.0 / 9.0 * t13533 + t13536 / 9.0 - 4.0 / 27.0 * t20292 + t20381 - t20382 + 22.0 / 9.0 * t20302 - 10.0 / 27.0 * t20305 + 4.0 / 3.0 * t20308 - 8.0 / 9.0 * t20312 - 2.0 / 9.0 * t20315 - 2.0 * t20318 + 8.0 / 3.0 * t20321 + 2.0 / 3.0 * t20324 - 2.0 / 3.0 * t20327;
    (t20361, t20365, t20373, t20392)
}
