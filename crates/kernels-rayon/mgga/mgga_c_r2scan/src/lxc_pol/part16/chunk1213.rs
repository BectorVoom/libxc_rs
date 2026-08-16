//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1213/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1213(t11717: f64, t26278: f64, t10760: f64, t29700: f64, t6085: f64, t38033: f64, t39996: f64, t40001: f64, t43459: f64, t43462: f64, t43465: f64, t43468: f64, t43471: f64, t43474: f64, t43477: f64) -> f64 {
    let t43480 = t26278 * t11717;
    let t43483 = t6085 * t10760 * t29700;
    let t43485 = 0.87327386630866483584e-2_f64 * t43459 + 0.13099107994629972538e-1_f64 * t43462 - 0.43663693315433241792e-2_f64 * t43465 + 0.13099107994629972538e-1_f64 * t43468 + 0.52396431978519890152e-1_f64 * t43471 + t39996 + 0.13972381860938637374e0_f64 * t40001 + 0.43341108700271342816e-1_f64 * t43474 - 0.43663693315433241792e-2_f64 * t43477 + 0.15573871527278325618e-1_f64 * t38033 - 0.43663693315433241792e-2_f64 * t43480 - 0.21831846657716620896e-2_f64 * t43483;
    t43485
}
