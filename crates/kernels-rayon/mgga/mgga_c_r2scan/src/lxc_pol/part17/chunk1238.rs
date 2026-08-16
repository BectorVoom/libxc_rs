//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1238/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1238(t41694: f64, t43495: f64, t43497: f64, t43500: f64, t43503: f64, t43506: f64, t43509: f64, t43512: f64, t43514: f64, t43516: f64, t43518: f64, t43521: f64) -> f64 {
    let t44424 = -0.87327386630866483588e-2_f64 * t43495 - 0.93149212406257582492e-1_f64 * t43497 + 0.17336443480108537126e0_f64 * t43500 - 0.46230515946956099003e0_f64 * t43503 + 0.17465477326173296718e-1_f64 * t43506 + 0.52396431978519890152e-1_f64 * t43509 + 0.32927245914677557992e0_f64 * t43512 + 0.34672886960217074252e0_f64 * t43514 + 0.52009330440325611378e0_f64 * t43516 + t41694 + 0.43663693315433241794e-2_f64 * t43518 - 0.46574606203128791246e-1_f64 * t43521;
    t44424
}
