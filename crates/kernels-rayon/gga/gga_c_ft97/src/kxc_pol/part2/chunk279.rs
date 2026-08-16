//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 279/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk279(t1096: f64, t680: f64, t203: f64, t222: f64, t205: f64, t207: f64, rho1: f64) -> (f64, f64, f64) {
    let t1097 = t680 * t1096;
    let t1100 = t203 * t222;
    let t1101 = t205 * rho1;
    let t1103 = 1.0_f64 / t207 / t1101;
    (t1097, t1100, t1103)
}
