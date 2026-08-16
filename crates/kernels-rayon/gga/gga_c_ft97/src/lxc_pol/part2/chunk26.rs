//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 26/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk26(t11: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t42 = t11 * sigma0;
    let t43 = rho0 * rho0;
    let t44 = pow_1_3(rho0);
    let t45 = t44 * t44;
    let t47 = 1.0_f64 / t45 / t43;
    (t42, t43, t44, t45, t47)
}
