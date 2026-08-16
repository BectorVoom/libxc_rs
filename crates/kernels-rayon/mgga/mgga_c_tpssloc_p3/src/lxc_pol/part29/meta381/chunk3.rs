//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1537/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1537(t10277: f64, t3061: f64, t14165: f64, t4582: f64, t12652: f64, t4588: f64, t12648: f64, t10216: f64, t10969: f64, t135: f64, t4608: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t14172 = t3061 * t10277;
    let t14173 = t14172 * t14165;
    let t14174 = t4582 * t14173;
    let t14179 = t4588 * t12652;
    let t14180 = t4582 * t14179;
    let t14183 = t4588 * t12648;
    let t14184 = t4582 * t14183;
    let t14187 = t10969 * t10216;
    let t14188 = t14187 * t14165;
    let t14189 = t4582 * t14188;
    let t14192 = t135 * t4608;
    let t14194 = t973 * t14192 / 432.0_f64;
    (t14174, t14180, t14184, t14189, t14194)
}
