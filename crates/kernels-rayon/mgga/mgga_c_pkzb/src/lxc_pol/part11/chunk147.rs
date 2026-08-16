//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 147/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk147(t16: f64, t23: f64, t434: f64, t441: f64, t445: f64, t7: f64) -> f64 {
    let t448 = -8.0_f64 / 3.0_f64 * t434 * t16 + 5.0_f64 / 3.0_f64 * t7 * t441 + 5.0_f64 / 3.0_f64 * t23 * t445;
    t448
}
