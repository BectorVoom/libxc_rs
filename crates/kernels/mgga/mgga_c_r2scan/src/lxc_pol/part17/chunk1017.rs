//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1017/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1017<F: Float>(t11036: F, t9657: F, t1070: F, t31764: F, t2928: F, t37028: F, t11033: F, t2938: F, t3366: F, t9640: F, t2441: F, t3675: F, t2983: F, t352: F, t856: F, t12574: F, t481: F) -> (F, F, F, F, F, F, F, F) {
    let t42534 = t11036 * t9657;
    let t42536 = t31764 * t1070;
    let t42539 = t37028 * t2928;
    let t42541 = t11033 * t2938;
    let t42543 = t9640 * t3366;
    let t42753 = t3675 * t2441;
    let t42757 = t2983 * t856 * t352;
    let t42819 = t12574 * t481;
    (t42534, t42536, t42539, t42541, t42543, t42753, t42757, t42819)
}
