//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1231/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1231(t40175: f64, t40177: f64, t40180: f64, t38123: f64, t38127: f64, t38134: f64, t38138: f64, t38646: f64, t40183: f64, t40185: f64, t40188: f64, t40191: f64) -> f64 {
    let t41734 = 0.46230515946956099004e0_f64 * t40175;
    let t41735 = 0.13869154784086829701e1_f64 * t40177;
    let t41736 = 0.13869154784086829701e1_f64 * t40180;
    let t41741 = -0.46574606203128791246e-1_f64 * t38123 - 0.13972381860938637374e0_f64 * t38127 - t38646 + 0.93149212406257582492e-1_f64 * t38134 + 0.55889527443754549496e0_f64 * t38138 + t41734 + t41735 + t41736 + 0.43663693315433241794e-2_f64 * t40183 - 0.31147743054556651237e-1_f64 * t40185 - 0.43663693315433241794e-2_f64 * t40188 - 0.13099107994629972538e-1_f64 * t40191;
    t41741
}
