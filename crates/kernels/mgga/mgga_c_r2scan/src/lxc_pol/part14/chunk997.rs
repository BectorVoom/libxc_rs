//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 997/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk997<F: Float>(t10710: F, t25486: F, t37582: F, t10776: F, t10810: F, t2563: F, t3308: F, t8102: F, t10772: F, t8106: F, t37782: F, t8111: F, t574: F, t7453: F, t2650: F, t546: F) -> (F, F, F, F, F, F, F) {
    let t39358 = t37582 * t10710 * t25486;
    let t39361 = t10776 * t10810 * t2563;
    let t39364 = t10776 * t3308 * t8102;
    let t39367 = t10772 * t3308 * t8106;
    let t39370 = t37782 * t3308 * t8111;
    let t39373 = t574 * t3308 * t7453;
    let t39375 = t546 * t2650;
    (t39358, t39361, t39364, t39367, t39370, t39373, t39375)
}
