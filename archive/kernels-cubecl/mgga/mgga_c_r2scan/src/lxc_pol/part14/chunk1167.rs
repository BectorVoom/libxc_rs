//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1167/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1167<F: Float>(t11015: F, t11568: F, t3434: F, t10680: F, t10681: F, t10683: F, t2482: F, t10673: F, t10674: F, t10676: F, t106: F, t7194: F, t97: F) -> (F, F, F, F) {
    let t40334 = t3434 * t11015 * t11568;
    let t40341 = t10680 * t10681 * t2482 * t10683;
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40358 = t97 * t106 * t7194;
    (t40334, t40341, t40345, t40358)
}
