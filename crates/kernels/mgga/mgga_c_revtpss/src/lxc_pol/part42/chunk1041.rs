//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1041/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1041<F: Float>(t14485: F, t2465: F, t1558: F, t836: F, t231: F, t2797: F, t2782: F, t860: F, t2783: F, t251: F, t4423: F, t10073: F, t4496: F, t10542: F, t4500: F, t4424: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t14486 = t2465 * t14485;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    let t14496 = t2797 * t14495;
    let t14498 = 0.10975748638225852664e-1 * t2782 * t14496;
    let t14502 = t860 * t1558;
    let t14504 = t2783 * t14502 * t231;
    let t14506 = 0.10975748638225852664e-1 * t2782 * t14504;
    let t14507 = t251 * t4423;
    let t14509 = t2783 * t14507 * t231;
    let t14511 = 0.10975748638225852664e-1 * t2782 * t14509;
    let t14512 = t10073 * t4496;
    let t14518 = 0.19514881078765566038e-1 * t10542 * t4500;
    let t14519 = t4424 * t72;
    (t14486, t14494, t14498, t14506, t14511, t14512, t14518, t14519)
}
