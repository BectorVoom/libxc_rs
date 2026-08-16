//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1217/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1217(t10868: f64, t2147: f64, t9292: f64, t40103: f64, t43495: f64, t43497: f64, t43500: f64, t43503: f64, t43506: f64, t43509: f64, t43512: f64, t43514: f64, t43516: f64, t43518: f64) -> f64 {
    let t43521 = t2147 * t10868 * t9292;
    let t43523 = -0.43663693315433241792e-2_f64 * t43495 - 0.46574606203128791245e-1_f64 * t43497 + 0.86682217400542685632e-1_f64 * t43500 - 0.23115257973478049502e0_f64 * t43503 + 0.87327386630866483584e-2_f64 * t43506 + 0.26198215989259945076e-1_f64 * t43509 + 0.16463622957338778997e0_f64 * t43512 + 0.17336443480108537126e0_f64 * t43514 + 0.2600466522016280569e0_f64 * t43516 + t40103 + 0.21831846657716620896e-2_f64 * t43518 - 0.23287303101564395623e-1_f64 * t43521;
    t43523
}
