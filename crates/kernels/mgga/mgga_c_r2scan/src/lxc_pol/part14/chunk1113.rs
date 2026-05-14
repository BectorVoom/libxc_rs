//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1113/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1113<F: Float>(t38211: F, t38216: F, t38220: F, t39106: F, t39107: F, t39108: F, t40587: F, t42253: F, t42255: F, t42257: F, t42260: F, t42265: F, t42267: F, t42270: F, t42274: F, t3579: F, t38723: F) -> (F, F) {
    let t42275 = -t42253 + t42255 - t42257 - t42260 - 0.16163010989689081288e-5 * t40587 + t42265 + t42267 - t42270 + 0.12195059916630011325e-2 * t38211 - 0.30487649791575028312e-3 * t38216 + 0.43368970657079495308e-4 * t38220 - t39106 - t39107 + t39108 + t42274;
    let t42277 = t3579 * t38723 / 2.0;
    (t42275, t42277)
}
