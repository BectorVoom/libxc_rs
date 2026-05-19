//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1226/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1226<F: Float>(t40041: F, t40044: F, t40047: F, t40050: F, t40053: F, t38036: F, t40024: F, t40027: F, t40029: F, t40031: F, t40035: F, t40038: F) -> F {
    let t41668 = F::cast_from(0.93149212406257582492e-1_f64) * t40041;
    let t41669 = F::cast_from(0.27944763721877274748e0_f64) * t40044;
    let t41670 = F::cast_from(0.93149212406257582492e-1_f64) * t40047;
    let t41671 = F::cast_from(0.27944763721877274748e0_f64) * t40050;
    let t41672 = F::cast_from(0.93149212406257582492e-1_f64) * t40053;
    let t41673 = -F::cast_from(0.17336443480108537126e0_f64) * t40024 - F::cast_from(0.86682217400542685632e-1_f64) * t40027 - F::cast_from(0.17336443480108537126e0_f64) * t40029 - F::cast_from(0.86682217400542685632e-1_f64) * t40031 - F::cast_from(0.87327386630866483588e-2_f64) * t40035 + F::cast_from(0.27944763721877274748e0_f64) * t38036 + F::cast_from(0.17336443480108537126e0_f64) * t40038 + t41668 + t41669 + t41670 + t41671 - t41672;
    t41673
}
