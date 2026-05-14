//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1120/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1120<F: Float>(t33121: F, t5323: F, t1873: F, t5327: F, t1800: F, t5332: F, t5336: F, t33089: F, t33092: F, t33095: F, t33099: F, t33101: F, t33104: F, t33107: F, t33110: F, t33112: F, t33114: F, t33116: F, t33118: F) -> (F, F, F, F, F) {
    let t33122 = t33121 * t5323;
    let t33124 = t1873 * t5327;
    let t33126 = t1800 * t5332;
    let t33128 = t1800 * t5336;
    let t33130 = t33089 / 16.0 - t33092 / 8.0 + t33095 / 12.0 + t33099 / 8.0 - t33101 / 12.0 - t33104 / 16.0 - t33107 / 72.0 + t33110 / 24.0 - t33112 / 128.0 + t33114 / 64.0 - t33116 / 48.0 - t33118 / 64.0 + t33122 / 48.0 + t33124 / 128.0 - t33126 / 288.0 - t33128 / 96.0;
    (t33122, t33124, t33126, t33128, t33130)
}
