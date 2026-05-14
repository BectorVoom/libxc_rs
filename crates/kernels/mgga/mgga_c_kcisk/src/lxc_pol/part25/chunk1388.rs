//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1388/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1388<F: Float>(t117265: F, t117267: F, t117434: F, t117526: F, t117548: F, t117556: F, t117557: F, t117565: F, t117568: F, t117574: F, t117582: F, t117586: F, t117591: F, t117597: F, t117601: F, t118541: F, t118543: F, t118548: F, t118549: F, t118556: F, t118558: F, t118559: F, t240: F) -> (F,) {
    let t118563 = t117265 - t117267 - t117434 + t240 * (t117526 + t117548 + t117591 + t118559) + t117556 - t117557 - t117565 - t117568 - t117574 + t117582 + t117586 - t117597 - t117601 - t118541 - t118543 + t118548 - t118549 + t118556 - t118558;
    (t118563,)
}
