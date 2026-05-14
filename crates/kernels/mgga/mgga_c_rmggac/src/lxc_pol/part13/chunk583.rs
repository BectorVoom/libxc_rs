//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 583/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk583<F: Float>(t7495: F, t7499: F, t7502: F, t7506: F, t7509: F, t7513: F, t2211: F, t4905: F, t884: F, t7519: F, t7522: F, t7525: F, t7544: F, t7549: F, t7582: F, t7594: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8109 = 0.40911992481368012596e-1 * t7495;
    let t8110 = 0.20455996240684006298e-1 * t7499;
    let t8111 = 0.5454932330849068346e-1 * t7502;
    let t8112 = 0.2727466165424534173e-1 * t7506;
    let t8113 = 0.13637330827122670865e-1 * t7509;
    let t8114 = 0.68186654135613354325e-2 * t7513;
    let t8115 = t2211 * t4905;
    let t8116 = t884 * t8115;
    let t8117 = 0.23948483403727617128e0 * t8116;
    let t8118 = 0.35922725105591425692e0 * t7519;
    let t8119 = 0.11974241701863808564e0 * t7522;
    let t8120 = 0.17961362552795712846e0 * t7525;
    let t8121 = 0.1702583995731913576e-4 * t7544;
    let t8122 = 0.85129199786595678799e-5 * t7549;
    let t8125 = 0.29568125932752208315e-3 * t7582;
    let t8129 = 0.22223798384940648817e-1 * t7594;
    (t8109, t8110, t8111, t8112, t8113, t8114, t8115, t8117, t8118, t8119, t8120, t8121, t8122, t8125, t8129)
}
