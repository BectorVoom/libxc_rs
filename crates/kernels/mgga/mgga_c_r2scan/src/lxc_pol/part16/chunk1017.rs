//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1017/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1017<F: Float>(t11033: F, t2938: F, t3366: F, t9640: F, t37039: F, t37041: F, t37066: F, t37076: F, t40822: F, t40841: F, t40845: F, t41872: F, t42524: F, t42526: F, t42528: F, t42530: F, t42532: F, t42534: F, t42536: F, t42539: F) -> (F,) {
    let t42541 = t11033 * t2938;
    let t42543 = t9640 * t3366;
    let t42546 = -3.0 / 4.0 * t42524 + t42526 / 4.0 + t42528 / 8.0 + t41872 + t40822 + 3.0 / 4.0 * t42530 - t42532 / 2.0 - t42534 / 4.0 - t42536 / 8.0 + 11.0 / 9.0 * t37041 + 2.0 / 3.0 * t42539 + t37039 - t40841 + t40845 - t42541 / 3.0 + t42543 / 3.0 - 11.0 / 9.0 * t37066 + t37076;
    (t42546,)
}
