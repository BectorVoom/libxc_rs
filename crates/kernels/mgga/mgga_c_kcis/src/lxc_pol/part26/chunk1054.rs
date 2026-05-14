//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1054/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1054<F: Float>(t2055: F, t5748: F, t2062: F, t5752: F, t2066: F, t1395: F, t7329: F, t7332: F, t4123: F, t7318: F, t28594: F, t8191: F, t7338: F, t7948: F, t29434: F, t29436: F, t29438: F, t29440: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29442 = t5748 * t2055;
    let t29444 = t5752 * t2062;
    let t29446 = t5752 * t2066;
    let t29448 = t1395 * t7329;
    let t29450 = t1395 * t7332;
    let t29452 = t4123 * t7318;
    let t29454 = t28594 * t8191;
    let t29456 = t7948 * t7338;
    let t29458 = t29434 / 8.0 - t29436 / 128.0 - t29438 / 12.0 + t29440 / 48.0 + t29442 / 64.0 + t29444 / 12.0 - t29446 / 48.0 - 19.0 / 72.0 * t29448 + t29450 / 9.0 - t29452 / 64.0 + t29454 / 3.0 - t29456 / 12.0;
    (t29442, t29444, t29446, t29448, t29450, t29452, t29454, t29456, t29458)
}
