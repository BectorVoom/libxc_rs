//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1125/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1125<F: Float>(t32636: F, t3373: F, t9368: F, t32633: F, t9365: F, t3422: F, t397: F, t9366: F) -> (F, F, F, F) {
    let t32637 = t3373 * t32636;
    let t32638 = t32637 * t9368;
    let t32640 = t9365 * t32633;
    let t32643 = t397 * t9366 * t3422;
    (t32637, t32638, t32640, t32643)
}
