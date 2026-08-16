//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1236/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1236(t38033: f64, t41649: f64, t41651: f64, t43459: f64, t43462: f64, t43465: f64, t43468: f64, t43471: f64, t43474: f64, t43477: f64, t43480: f64, t43483: f64) -> f64 {
    let t44407 = 0.17465477326173296718e-1_f64 * t43459 + 0.26198215989259945076e-1_f64 * t43462 - 0.87327386630866483588e-2_f64 * t43465 + 0.26198215989259945076e-1_f64 * t43468 + 0.1047928639570397803e0_f64 * t43471 + t41649 + t41651 + 0.86682217400542685632e-1_f64 * t43474 - 0.87327386630866483588e-2_f64 * t43477 + 0.31147743054556651237e-1_f64 * t38033 - 0.87327386630866483588e-2_f64 * t43480 - 0.43663693315433241794e-2_f64 * t43483;
    t44407
}
