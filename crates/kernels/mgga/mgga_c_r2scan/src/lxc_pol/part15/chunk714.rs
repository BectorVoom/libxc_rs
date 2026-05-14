//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 714/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk714<F: Float>(t510: F, t116: F, t122: F, t3436: F, t57: F, t2116: F, t6161: F, t1266: F, t277: F) -> (F, F, F) {
    let t6325 = t510 * t510;
    let t6326 = 1.0 / t6325;
    let t6327 = t116 * t6326;
    let t6329 = t122 * t3436 * t57;
    let t6331 = t2116 * t6161;
    let t6333 = 0.25705033881751801528e-4 * t6327 * t6329 * t6331;
    let t6343 = t1266 * t277;
    (t6327, t6333, t6343)
}
