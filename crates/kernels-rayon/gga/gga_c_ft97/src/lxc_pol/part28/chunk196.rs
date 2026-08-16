//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 196/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk196(t140: f64, t1291: f64, t1303: f64, t1355: f64, t543: f64) -> f64 {
    let t141 = 0.1e-59_f64 < t140;
    let t1359 = piecewise3(t141, 0.22653425206514361674e0_f64 * t543 * t1291 - 0.22653425206514361674e0_f64 * t140 * t1291 - 0.50008500819444444447e-1_f64 * t1355 * t1303, 0.0_f64);
    t1359
}
