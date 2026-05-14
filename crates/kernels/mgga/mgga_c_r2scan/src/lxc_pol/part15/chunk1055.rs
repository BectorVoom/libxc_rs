//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1055/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1055<F: Float>(t40177: F, t261: F, t3304: F, t7309: F, t10760: F, t2147: F, t24059: F, t10740: F, t980: F, t24750: F, t6085: F, t24070: F, t6093: F, t38123: F, t38127: F, t38131: F, t38134: F, t38138: F, t40176: F) -> (F,) {
    let t40178 = 0.69345773920434148506e0 * t40177;
    let t40180 = t3304 * t261 * t7309;
    let t40181 = 0.69345773920434148506e0 * t40180;
    let t40183 = t2147 * t10760 * t24059;
    let t40185 = t980 * t10740;
    let t40188 = t6085 * t10760 * t24750;
    let t40191 = t6093 * t10760 * t24070;
    let t40193 = -0.23287303101564395623e-1 * t38123 - 0.69861909304693186869e-1 * t38127 - t38131 + 0.46574606203128791246e-1 * t38134 + 0.27944763721877274748e0 * t38138 + t40176 + t40178 + t40181 + 0.21831846657716620896e-2 * t40183 - 0.15573871527278325618e-1 * t40185 - 0.21831846657716620896e-2 * t40188 - 0.65495539973149862688e-2 * t40191;
    (t40193,)
}
