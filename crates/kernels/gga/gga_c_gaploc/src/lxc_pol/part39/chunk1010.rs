//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1010/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1010<F: Float>(t42034: F, t42038: F, t42042: F, t42047: F, t42051: F, t42054: F, t42059: F, t42064: F, t42067: F, t42069: F, t42072: F, t42074: F, t40332: F, t40336: F, t1457: F, t1572: F, t47026: F) -> (F, F, F, F) {
    let t48044 = 0.35750489951850426669e0 * t42034 + 0.1022478025437886658e1 * t42038 + 0.15337170381568299871e1 * t42042 + t42047 + t42051 - 0.25561950635947166451e1 * t42054 - t42059 - t42064 + t42067 - t42069 + t42072 + 0.71500979903700853338e0 * t42074;
    let t48047 = 0.15337170381568299871e1 * t40332;
    let t48048 = 0.38342925953920749677e0 * t40336;
    let t48050 = t1572 * t1457 * t47026;
    (t48044, t48047, t48048, t48050)
}
