//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 946/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk946(t22857: f64, t543: f64, t1390: f64, t828: f64, t22762: f64, t22763: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64) {
    let t22912 = t22857 * t543;
    let t22914 = t1390 * t828 * t22912;
    let t22917 = t22762 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t9389 - t9391 - t22763;
    (t22912, t22914, t22917)
}
