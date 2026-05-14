//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 993/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk993<F: Float>(t10879: F, t11727: F, t261: F, t3304: F, t7309: F, t10740: F, t980: F, t29418: F, t3293: F, t132: F, t537: F, t1575: F, t25826: F, t3342: F, t571: F, t10856: F, t8071: F) -> (F, F, F, F, F, F, F) {
    let t40177 = t10879 * t11727;
    let t40180 = t3304 * t261 * t7309;
    let t40185 = t980 * t10740;
    let t40194 = t3293 * t29418;
    let t40195 = t132 * t537;
    let t40201 = t571 * t1575 * t3342 * t25826;
    let t40215 = t10856 * t8071;
    (t40177, t40180, t40185, t40194, t40195, t40201, t40215)
}
