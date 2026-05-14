//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1320/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1320<F: Float>(t32266: F, t471: F, t97: F, t19061: F, t19069: F, t19341: F, t23781: F, t23798: F, t23800: F, t32217: F, t32218: F, t32219: F, t32222: F, t32225: F, t32228: F, t537: F, t9937: F) -> (F, F, F) {
    let t32269 = 3.0 * t97 * t471 * t32266;
    let t32270 = -t19061 + t32217 - t32218 - t19069 + t23781 + t19341 + t32219 - t32222 - t23798 + t32225 + t32228 + t23800 - t32269;
    let t32309 = t537 * t9937;
    (t32269, t32270, t32309)
}
