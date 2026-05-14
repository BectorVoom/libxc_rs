//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1056/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1056<F: Float>(t573: F, t7381: F, t1395: F, t7287: F, t4261: F, t7202: F, t7952: F, t2034: F, t2043: F, t28640: F, t8196: F, t29459: F, t29461: F, t29463: F, t29466: F, t29468: F, t29471: F, t29473: F) -> (F, F, F, F, F, F, F) {
    let t29475 = t7381 * t573;
    let t29477 = t1395 * t7287;
    let t29479 = t4261 * t7202;
    let t29480 = t7952 * t29479;
    let t29482 = t2034 * t2043;
    let t29484 = t28640 * t8196;
    let t29486 = t29459 / 128.0 + 11.0 / 18.0 * t29461 - 2.0 / 9.0 * t29463 - t29466 / 16.0 - t29468 / 8.0 - t29471 / 72.0 - t29473 / 288.0 + t29475 / 16.0 - t29477 / 96.0 + t29480 / 24.0 - t29482 / 3.0 + t29484 / 12.0;
    (t29475, t29477, t29479, t29480, t29482, t29484, t29486)
}
