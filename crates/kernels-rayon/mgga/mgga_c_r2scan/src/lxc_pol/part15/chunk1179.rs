//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1179/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1179(t2147: f64, t26307: f64, t3332: f64, t38096: f64, t38099: f64, t38111: f64, t38114: f64, t40145: f64, t40149: f64, t40151: f64, t40153: f64, t40156: f64, t40158: f64, t40162: f64) -> f64 {
    let t40165 = t2147 * t3332 * t26307;
    let t40167 = 0.21831846657716620896e-2_f64 * t40145 - 0.46574606203128791246e-1_f64 * t38096 - 0.13972381860938637374e0_f64 * t38099 - 0.87327386630866483584e-2_f64 * t40149 - 0.13099107994629972538e-1_f64 * t40151 + 0.43663693315433241792e-2_f64 * t40153 - t40156 + t40158 - 0.12805040077930161442e0_f64 * t38111 - 0.23115257973478049502e0_f64 * t38114 - 0.13972381860938637373e0_f64 * t40162 + 0.21831846657716620896e-2_f64 * t40165;
    t40167
}
