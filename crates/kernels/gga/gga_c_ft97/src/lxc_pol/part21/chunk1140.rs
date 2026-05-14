//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1140/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1140<F: Float>(t46727: F, t6547: F, t102053: F, t108: F, t116082: F, t116091: F, t116093: F, t116095: F, t116097: F, t116099: F, t116102: F, t116105: F, t116108: F, t1286: F, t1564: F, t22917: F, t28: F, t29572: F, t369: F, t4458: F, t5495: F, t5501: F) -> (F, F) {
    let t116110 = t46727 * t6547;
    let t116112 = t5501 * t1564 * t22917 * t4458 / 9.0 - t102053 + t1286 * t28 * t369 * t116082 * t108 / 6.0 + t5495 * t29572 / 6.0 + 2.0 / 9.0 * t116091 - 12.0 * t116093 + 8.0 * t116095 + 8.0 * t116097 - 4.0 * t116099 + 8.0 * t116102 + 4.0 * t116105 + 4.0 * t116108 + 8.0 * t116110;
    (t116110, t116112)
}
