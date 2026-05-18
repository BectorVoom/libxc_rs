//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1171/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1171<F: Float>(t10752: F, t30370: F, t38145: F, t6085: F, t7922: F, t6093: F, t7605: F, t8081: F, t7619: F, t2147: F, t7624: F, t38036: F, t40024: F, t40027: F, t40029: F, t40031: F, t40035: F) -> F {
    let t40038 = t30370 * t10752;
    let t40041 = t6085 * t38145 * t7922;
    let t40042 = F::new(0.46574606203128791246e-1) * t40041;
    let t40044 = t6093 * t38145 * t7605;
    let t40047 = t6085 * t38145 * t8081;
    let t40048 = F::new(0.46574606203128791246e-1) * t40047;
    let t40050 = t6093 * t38145 * t7619;
    let t40051 = F::new(0.13972381860938637374e0) * t40050;
    let t40053 = t2147 * t38145 * t7624;
    let t40054 = F::new(0.46574606203128791246e-1) * t40053;
    let t40055 = -F::new(0.86682217400542685632e-1) * t40024 - F::new(0.43341108700271342816e-1) * t40027 - F::new(0.86682217400542685632e-1) * t40029 - F::new(0.43341108700271342816e-1) * t40031 - F::new(0.43663693315433241792e-2) * t40035 + F::new(0.13972381860938637374e0) * t38036 + F::new(0.86682217400542685632e-1) * t40038 + t40042 + F::new(0.13972381860938637373e0) * t40044 + t40048 + t40051 - t40054;
    t40055
}
