//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1181/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1181<F: Float>(t10760: F, t24750: F, t6085: F, t24070: F, t6093: F, t38123: F, t38127: F, t38131: F, t38134: F, t38138: F, t40176: F, t40178: F, t40181: F, t40183: F, t40185: F) -> F {
    let t40188 = t6085 * t10760 * t24750;
    let t40191 = t6093 * t10760 * t24070;
    let t40193 = -F::new(0.23287303101564395623e-1) * t38123 - F::new(0.69861909304693186869e-1) * t38127 - t38131 + F::new(0.46574606203128791246e-1) * t38134 + F::new(0.27944763721877274748e0) * t38138 + t40176 + t40178 + t40181 + F::new(0.21831846657716620896e-2) * t40183 - F::new(0.15573871527278325618e-1) * t40185 - F::new(0.21831846657716620896e-2) * t40188 - F::new(0.65495539973149862688e-2) * t40191;
    t40193
}
