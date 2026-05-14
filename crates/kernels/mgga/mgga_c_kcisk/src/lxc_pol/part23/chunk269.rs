//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 269/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk269<F: Float>(t1237: F, t1242: F, t164: F, t313: F, t353: F, t352: F, t81: F) -> (F, F, F, F) {
    let t1243 = t1242 * t1237;
    let t1246 = t353 * t164 * t313;
    let t1247 = 0.16431333333333333333e0 * t1246;
    let t1248 = t352 * t81;
    (t1243, t1246, t1247, t1248)
}
