//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 954/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk954<F: Float>(t11479: F, t3275: F, t3352: F, t3270: F, t3618: F, t3269: F, t3574: F, t792: F, t3276: F, t3262: F, t10918: F, t3263: F, t7040: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11481 = t3275 * t11479 * t3352;
    let t11482 = t11481 / F::cast_from(4.0_f64);
    let t11483 = t3270 * t3618;
    let t11484 = t3269 * t11483;
    let t11485 = t11484 / F::cast_from(4.0_f64);
    let t11486 = t3574 * t792;
    let t11487 = t3276 * t11486;
    let t11488 = t3262 * t11487;
    let t11489 = F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t11488;
    let t11491 = t3262 * t10918 * t3574;
    let t11492 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t11491;
    let t11494 = t3275 * t3263 * t7040;
    (t11481, t11482, t11483, t11484, t11485, t11486, t11487, t11488, t11489, t11491, t11492, t11494)
}
