//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1171/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1171(t10752: f64, t30370: f64, t38145: f64, t6085: f64, t7922: f64, t6093: f64, t7605: f64, t8081: f64, t7619: f64, t2147: f64, t7624: f64, t38036: f64, t40024: f64, t40027: f64, t40029: f64, t40031: f64, t40035: f64) -> f64 {
    let t40038 = t30370 * t10752;
    let t40041 = t6085 * t38145 * t7922;
    let t40042 = 0.46574606203128791246e-1_f64 * t40041;
    let t40044 = t6093 * t38145 * t7605;
    let t40047 = t6085 * t38145 * t8081;
    let t40048 = 0.46574606203128791246e-1_f64 * t40047;
    let t40050 = t6093 * t38145 * t7619;
    let t40051 = 0.13972381860938637374e0_f64 * t40050;
    let t40053 = t2147 * t38145 * t7624;
    let t40054 = 0.46574606203128791246e-1_f64 * t40053;
    let t40055 = -0.86682217400542685632e-1_f64 * t40024 - 0.43341108700271342816e-1_f64 * t40027 - 0.86682217400542685632e-1_f64 * t40029 - 0.43341108700271342816e-1_f64 * t40031 - 0.43663693315433241792e-2_f64 * t40035 + 0.13972381860938637374e0_f64 * t38036 + 0.86682217400542685632e-1_f64 * t40038 + t40042 + 0.13972381860938637373e0_f64 * t40044 + t40048 + t40051 - t40054;
    t40055
}
