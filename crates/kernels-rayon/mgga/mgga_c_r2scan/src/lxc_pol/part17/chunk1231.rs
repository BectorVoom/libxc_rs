//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1231/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1231(t37933: f64, t43313: f64, t43316: f64, t43319: f64, t43322: f64, t43324: f64, t43327: f64, t43330: f64, t43332: f64, t43335: f64, t43337: f64, t43340: f64) -> f64 {
    let t44343 = -0.42683466926433871473e0_f64 * t37933 + 0.23115257973478049502e0_f64 * t43313 + 0.27944763721877274748e0_f64 * t43316 + 0.26198215989259945076e-1_f64 * t43319 + 0.93149212406257582492e-1_f64 * t43322 + 0.10401866088065122276e1_f64 * t43324 + 0.87327386630866483588e-2_f64 * t43327 - 0.86682217400542685632e-1_f64 * t43330 - 0.5200933044032561138e0_f64 * t43332 + 0.43663693315433241794e-2_f64 * t43335 + 0.13869154784086829701e1_f64 * t43337 + 0.52396431978519890152e-1_f64 * t43340;
    t44343
}
