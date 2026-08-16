//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1493/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1493(t14502: f64, t231: f64, t2783: f64, t2782: f64, t251: f64, t4423: f64, t10073: f64, t4496: f64, t10542: f64, t4500: f64, t4424: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14504 = t2783 * t14502 * t231;
    let t14506 = 0.10975748638225852664e-1_f64 * t2782 * t14504;
    let t14507 = t251 * t4423;
    let t14509 = t2783 * t14507 * t231;
    let t14511 = 0.10975748638225852664e-1_f64 * t2782 * t14509;
    let t14512 = t10073 * t4496;
    let t14518 = 0.19514881078765566038e-1_f64 * t10542 * t4500;
    let t14519 = t4424 * t72;
    (t14506, t14507, t14511, t14512, t14518, t14519)
}
