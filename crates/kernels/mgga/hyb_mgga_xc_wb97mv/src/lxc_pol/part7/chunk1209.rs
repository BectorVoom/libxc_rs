//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1209/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1209<F: Float>(t29125: F, t2990: F, t10525: F, t1178: F, t1993: F, t21339: F, t21342: F, t25298: F, t25301: F, t25306: F, t25317: F, t25352: F, t25357: F, t25360: F, t25373: F, t25452: F, t29097: F, t29099: F, t29105: F, t29109: F, t29112: F, t29119: F, t8140: F, t8440: F) -> (F, F) {
    let t29126 = t2990 * t29125;
    let t29132 = -t29097 / 32.0 - t29099 / 16.0 + t25298 / 72.0 + t25301 / 72.0 - 41.0 / 144.0 * t25306 - t29105 / 72.0 - t25317 / 96.0 - t29109 / 96.0 + t29112 / 216.0 - t1993 * t25352 * t1178 / 12.0 - t25357 / 48.0 + t29119 / 288.0 - t25360 / 48.0 - 5.0 / 432.0 * t25373 + t21339 / 96.0 + t21342 / 48.0 - 7.0 / 18.0 * t8140 * t25452 * t29126 - 3.0 / 16.0 * t8440 * t10525;
    (t29126, t29132)
}
