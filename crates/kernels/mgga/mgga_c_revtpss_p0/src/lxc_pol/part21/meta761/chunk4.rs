//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2700/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2700<F: Float>(t1353: F, t4135: F, t14304: F, t1450: F, t1448: F, t47109: F, t47116: F, t47118: F, t47122: F, t47124: F, t48315: F, t48316: F, t48317: F, t48318: F, t48319: F, t48320: F) -> (F, F, F, F) {
    let t49640 = t4135 * t1353;
    let t49647 = t14304 * t1450;
    let t49654 = t1448 * t4135;
    let t49659 = -t48315 - t47109 - t48316 + t48317 + t47116 - t47118 - t48318 + t47122 + t47124 + t48319 + t48320;
    (t49640, t49647, t49654, t49659)
}
