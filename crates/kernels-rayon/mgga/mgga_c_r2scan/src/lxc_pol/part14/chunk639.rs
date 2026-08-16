//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 639/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk639(t3450: f64, t3455: f64, t3459: f64, t3468: f64, t3471: f64, t3475: f64, t3496: f64, t3499: f64, t3564: f64, t3565: f64, t3566: f64, t797: f64, t910: f64) -> (f64, f64, f64) {
    let t3567 = 0.30487649791575028312e-3_f64 * t3450;
    let t3570 = -t3564 + t3565 - t3566 - t3567 - 0.72042316457491791901e-3_f64 * t3455 + 0.30487649791575028312e-3_f64 * t3459 - t3468 - t3471 + t3475 - t3496 + t3499;
    let t3574 = t797 * t910;
    (t3567, t3570, t3574)
}
