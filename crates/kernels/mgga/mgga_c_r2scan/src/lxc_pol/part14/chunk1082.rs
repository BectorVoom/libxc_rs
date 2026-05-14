//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1082/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1082<F: Float>(t40041: F, t40044: F, t40047: F, t40050: F, t40053: F, t38036: F, t40024: F, t40027: F, t40029: F, t40031: F, t40035: F, t40038: F, t40070: F, t40076: F, t40086: F, t38056: F, t38062: F, t38617: F, t40059: F, t40064: F, t40068: F, t40073: F, t40081: F, t40084: F) -> (F, F) {
    let t41668 = 0.93149212406257582492e-1 * t40041;
    let t41669 = 0.27944763721877274748e0 * t40044;
    let t41670 = 0.93149212406257582492e-1 * t40047;
    let t41671 = 0.27944763721877274748e0 * t40050;
    let t41672 = 0.93149212406257582492e-1 * t40053;
    let t41673 = -0.17336443480108537126e0 * t40024 - 0.86682217400542685632e-1 * t40027 - 0.17336443480108537126e0 * t40029 - 0.86682217400542685632e-1 * t40031 - 0.87327386630866483588e-2 * t40035 + 0.27944763721877274748e0 * t38036 + 0.17336443480108537126e0 * t40038 + t41668 + t41669 + t41670 + t41671 - t41672;
    let t41680 = 0.11902492299418487743e0 * t40070;
    let t41682 = 0.95219938395347901946e-2 * t40076;
    let t41687 = 0.93149212406257582492e-1 * t40086;
    let t41688 = 0.17336443480108537126e0 * t40059 + 0.17465477326173296718e-1 * t40064 + 0.52396431978519890152e-1 * t40068 - t41680 + 0.2600466522016280569e0 * t40073 + t41682 - t38617 - 0.23115257973478049502e0 * t38056 + 0.93149212406257582492e-1 * t38062 - 0.26198215989259945076e-1 * t40081 - 0.1047928639570397803e0 * t40084 - t41687;
    (t41673, t41688)
}
