//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1084/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1084<F: Float>(t40155: F, t40157: F, t40162: F, t38096: F, t38099: F, t38111: F, t38114: F, t40145: F, t40149: F, t40151: F, t40153: F, t40165: F, t40175: F, t40177: F, t40180: F, t38123: F, t38127: F, t38134: F, t38138: F, t38646: F, t40183: F, t40185: F, t40188: F, t40191: F) -> (F, F) {
    let t41721 = 0.93149212406257582492e-1 * t40155;
    let t41722 = 0.46230515946956099004e0 * t40157;
    let t41725 = 0.27944763721877274748e0 * t40162;
    let t41727 = 0.43663693315433241794e-2 * t40145 - 0.93149212406257582492e-1 * t38096 - 0.27944763721877274748e0 * t38099 - 0.17465477326173296718e-1 * t40149 - 0.26198215989259945076e-1 * t40151 + 0.87327386630866483588e-2 * t40153 - t41721 + t41722 - 0.25610080155860322884e0 * t38111 - 0.46230515946956099004e0 * t38114 - t41725 + 0.43663693315433241794e-2 * t40165;
    let t41734 = 0.46230515946956099004e0 * t40175;
    let t41735 = 0.13869154784086829701e1 * t40177;
    let t41736 = 0.13869154784086829701e1 * t40180;
    let t41741 = -0.46574606203128791246e-1 * t38123 - 0.13972381860938637374e0 * t38127 - t38646 + 0.93149212406257582492e-1 * t38134 + 0.55889527443754549496e0 * t38138 + t41734 + t41735 + t41736 + 0.43663693315433241794e-2 * t40183 - 0.31147743054556651237e-1 * t40185 - 0.43663693315433241794e-2 * t40188 - 0.13099107994629972538e-1 * t40191;
    (t41727, t41741)
}
