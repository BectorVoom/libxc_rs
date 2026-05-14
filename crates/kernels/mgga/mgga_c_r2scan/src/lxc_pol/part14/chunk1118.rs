//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1118/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1118<F: Float>(t38334: F, t38339: F, t38356: F, t38359: F, t39122: F, t39127: F, t39129: F, t39130: F, t39131: F, t39134: F, t42330: F, t42334: F, t42339: F, t42344: F, t42346: F, t42349: F) -> (F,) {
    let t42350 = -t42330 + t42334 + t39122 - 0.30487649791575028312e-3 * t38334 + t39127 + 0.325201597776800302e-2 * t38339 - t39129 + t39130 - t39131 - t42339 - 0.76845137554657911361e-2 * t38356 + 0.12195059916630011325e-2 * t38359 + t39134 + t42344 - t42346 + t42349;
    (t42350,)
}
