//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 919/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk919(t31597: f64, t31539: f64, t368: f64, t7457: f64, t7458: f64, t7310: f64, t7386: f64, t7637: f64, t7753: f64, t3077: f64, t7646: f64, t1167: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31598 = 0.10718504529517434243e-3_f64 * t31597;
    let t31601 = t7457 * t7458 * t368 * t31539;
    let t31602 = 0.21437009059034868486e-3_f64 * t31601;
    let t31603 = t7310 * t7386;
    let t31605 = t7637 * t7753;
    let t31611 = t3077 * t7646;
    let t31612 = t31611 * t1167;
    (t31598, t31602, t31603, t31605, t31611, t31612)
}
