//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 159/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk159(t38: f64, t627: f64, t45: f64, t78: f64, t57: f64, t81: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t628 = t38 * t627;
    let t631 = t45 * t45;
    let t633 = 1.0_f64 / t78 / t631;
    let t635 = t57 * t57;
    let t637 = 1.0_f64 / t81 / t635;
    let t640 = -4.0_f64 / 3.0_f64 * t633 * t606 + 4.0_f64 / 3.0_f64 * t637 * t606;
    (t628, t631, t633, t635, t637, t640)
}
