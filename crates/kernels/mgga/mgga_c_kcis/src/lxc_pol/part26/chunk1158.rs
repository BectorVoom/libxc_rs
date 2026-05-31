//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1158/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1158<F: Float>(t2066: F, t5752: F, t1395: F, t7329: F, t7332: F, t4123: F, t7318: F, t28594: F, t8191: F, t7338: F, t7948: F, t29434: F, t29436: F, t29438: F, t29440: F, t29442: F, t29444: F) -> (F, F, F, F, F, F, F) {
    let t29446 = t5752 * t2066;
    let t29448 = t1395 * t7329;
    let t29450 = t1395 * t7332;
    let t29452 = t4123 * t7318;
    let t29454 = t28594 * t8191;
    let t29456 = t7948 * t7338;
    let t29458 = t29434 / F::cast_from(8.0_f64) - t29436 / F::cast_from(128.0_f64) - t29438 / F::cast_from(12.0_f64) + t29440 / F::cast_from(48.0_f64) + t29442 / F::cast_from(64.0_f64) + t29444 / F::cast_from(12.0_f64) - t29446 / F::cast_from(48.0_f64) - F::cast_from(19.0_f64) / F::cast_from(72.0_f64) * t29448 + t29450 / F::cast_from(9.0_f64) - t29452 / F::cast_from(64.0_f64) + t29454 / F::cast_from(3.0_f64) - t29456 / F::cast_from(12.0_f64);
    (t29446, t29448, t29450, t29452, t29454, t29456, t29458)
}
