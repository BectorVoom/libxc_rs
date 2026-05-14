//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 930/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk930<F: Float>(t1269: F, t818: F, t1275: F, t815: F, t817: F, t312: F, t1277: F, t826: F, t1289: F, t317: F, t6100: F, t313: F, t6623: F, t6627: F, t6631: F, t6637: F, t6641: F, t6645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6651 = t1269 * t818;
    let t6654 = t815 * t1275;
    let t6659 = t817 * t817;
    let t6660 = 1.0 / t6659;
    let t6661 = t312 * t6660;
    let t6662 = t1277 * t826;
    let t6665 = t826 * t1289;
    let t6678 = 154.0 / 27.0 * t317 * t6100;
    let t6679 = 3.0 / 10.0 * t313 * (-10.0 / 27.0 * t6623 + 10.0 / 3.0 * t6627 + 5.0 / 3.0 * t6631 - 10.0 / 27.0 * t6637 + 10.0 / 3.0 * t6641 + 5.0 / 3.0 * t6645) - t6678;
    (t6651, t6654, t6659, t6660, t6661, t6662, t6665, t6678, t6679)
}
