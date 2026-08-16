//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 380/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk380(t1266: f64, t136: f64, t191: f64, t507: f64, t604: f64, t22: f64, t643: f64) -> (f64, f64, f64, f64) {
    let t1754 = t1266 * t136;
    let t1755 = t1754 * t191;
    let t1758 = t604 * t507;
    let t1762 = 1.0_f64 / t22 / t643;
    (t1754, t1755, t1758, t1762)
}
