//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 299/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk299<F: Float>(t513: F, t295: F, t299: F, t305: F, t803: F, t807: F, t811: F, t320: F) -> (F, F, F, F) {
    let t814 = t513 / 3.0;
    let t815 = -5.0 / 3.0 * t803 * t299 + 5.0 / 3.0 * t295 * t807 + 5.0 / 3.0 * t305 * t811 + t814;
    let t817 = t320 * t320;
    let t818 = 1.0 / t817;
    (t814, t815, t817, t818)
}
