//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 150/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk150(t6: f64, t16: f64, t34: f64, t38: f64, t441: f64, t445: f64, tau0: f64) -> (f64, f64) {
    let t454 = tau0 * t6;
    let t459 = -5.0_f64 / 3.0_f64 * t454 * t16 + 5.0_f64 / 3.0_f64 * t34 * t441 + 5.0_f64 / 3.0_f64 * t38 * t445;
    (t454, t459)
}
