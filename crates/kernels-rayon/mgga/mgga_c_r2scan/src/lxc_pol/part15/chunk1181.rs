//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1181/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1181(t10760: f64, t24750: f64, t6085: f64, t24070: f64, t6093: f64, t38123: f64, t38127: f64, t38131: f64, t38134: f64, t38138: f64, t40176: f64, t40178: f64, t40181: f64, t40183: f64, t40185: f64) -> f64 {
    let t40188 = t6085 * t10760 * t24750;
    let t40191 = t6093 * t10760 * t24070;
    let t40193 = -0.23287303101564395623e-1_f64 * t38123 - 0.69861909304693186869e-1_f64 * t38127 - t38131 + 0.46574606203128791246e-1_f64 * t38134 + 0.27944763721877274748e0_f64 * t38138 + t40176 + t40178 + t40181 + 0.21831846657716620896e-2_f64 * t40183 - 0.15573871527278325618e-1_f64 * t40185 - 0.21831846657716620896e-2_f64 * t40188 - 0.65495539973149862688e-2_f64 * t40191;
    t40193
}
