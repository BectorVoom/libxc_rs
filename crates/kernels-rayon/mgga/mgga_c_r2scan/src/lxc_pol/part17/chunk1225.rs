//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1225/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1225(t37700: f64, t37707: f64, t39579: f64, t41464: f64, t41466: f64, t43178: f64, t43181: f64, t43183: f64, t43185: f64, t43188: f64, t43191: f64, t39635: f64, t39637: f64, t39642: f64, t39672: f64, t41474: f64, t41475: f64, t41479: f64, t43195: f64, t43200: f64, t43203: f64, t43205: f64, t43209: f64) -> (f64, f64) {
    let t44268 = -0.51220160311720645768e0_f64 * t39579 + 0.11708928647259339623e0_f64 * t37700 - 0.45022119329691164871e0_f64 * t37707 + 0.52396431978519890152e-1_f64 * t43178 - 0.13099107994629972538e-1_f64 * t43181 - 0.87327386630866483588e-2_f64 * t43183 - 0.26198215989259945076e-1_f64 * t43185 + t41464 + 0.13099107994629972538e-1_f64 * t43188 - t41466 - 0.13972381860938637374e0_f64 * t43191;
    let t44278 = 0.26198215989259945076e-1_f64 * t43195 + t41474 + t41475 - 0.50853567541651708904e1_f64 * t39635 - 0.65854491829355115985e-1_f64 * t39637 - t41479 + 0.23417857294518679244e0_f64 * t39642 + 0.26198215989259945076e-1_f64 * t43200 - 0.17465477326173296718e-1_f64 * t43203 - 0.51220160311720645768e0_f64 * t39672 - 0.26198215989259945076e-1_f64 * t43205 + 0.13099107994629972538e-1_f64 * t43209;
    (t44268, t44278)
}
