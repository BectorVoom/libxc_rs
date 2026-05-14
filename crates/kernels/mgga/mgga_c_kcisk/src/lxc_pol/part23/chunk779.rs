//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 779/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk779<F: Float>(t9497: F, t9498: F, t3512: F, t500: F, t1415: F, t1506: F, t1340: F, t1513: F, t1517: F, t9489: F, t9493: F, t9495: F) -> (F, F, F, F, F, F) {
    let t9499 = t9497 * t9498;
    let t9501 = t3512 * t500;
    let t9503 = t1415 * t1506;
    let t9505 = t1340 * t1513;
    let t9507 = t1340 * t1517;
    let t9509 = t9489 / 16.0 - t9493 / 16.0 - t9495 / 6.0 + t9499 / 24.0 - t9501 / 128.0 + t9503 / 128.0 + t9505 / 24.0 - t9507 / 96.0;
    (t9499, t9501, t9503, t9505, t9507, t9509)
}
