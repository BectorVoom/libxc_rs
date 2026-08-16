//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 677/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk677(t285: f64, t282: f64) -> (f64, f64) {
    let t6849 = t285 * t285;
    let t6851 = 1.0_f64 / t6849 / t282;
    (t6849, t6851)
}
