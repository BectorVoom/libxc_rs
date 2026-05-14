//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1040/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1040<F: Float>(t2768: F, t874: F, t10680: F, t10682: F, t10978: F, t10980: F, t11568: F, t122: F, t10673: F, t10675: F, t10954: F, t11564: F, t3446: F, t11015: F, t3434: F, t10681: F, t10683: F, t2482: F) -> (F, F, F, F, F, F) {
    let t40310 = t2768 * t874;
    let t40312 = t10680 * t10682 * t40310;
    let t40315 = t10978 * t10980 * t11568;
    let t40317 = t2768 * t122;
    let t40319 = t10673 * t10675 * t40317;
    let t40331 = t3446 * t10954 * t11564;
    let t40334 = t3434 * t11015 * t11568;
    let t40341 = t10680 * t10681 * t2482 * t10683;
    (t40312, t40315, t40319, t40331, t40334, t40341)
}
