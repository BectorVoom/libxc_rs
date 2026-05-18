//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 607/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk607<F: Float>(t3347: F, t797: F, t1048: F, t499: F, t2333: F, t795: F, t3263: F, t3275: F, t321: F, t502: F, t263: F, t818: F) -> (F, F, F, F, F, F) {
    let t3348 = t3347 * t797;
    let t3350 = t1048 * t499 * t3348;
    let t3351 = t3350 / F::new(4.0);
    let t3352 = t2333 * t795;
    let t3354 = t3275 * t3263 * t3352;
    let t3355 = t3354 / F::new(4.0);
    let t3356 = t502 * t321;
    let t3357 = t3356 / F::new(3.0);
    let t3358 = t263 * t818;
    (t3348, t3351, t3352, t3355, t3357, t3358)
}
