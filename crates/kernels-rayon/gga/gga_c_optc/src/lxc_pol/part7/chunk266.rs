//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 266/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk266(t779: f64, t800: f64, t765: f64, t772: f64) -> (f64, f64) {
    let t802 = 1.0_f64 * t779 * t800;
    let t803 = 0.17123333333333333333e-1_f64 * t765;
    let t805 = -t803 - 0.17123333333333333333e-1_f64 * t772;
    (t802, t805)
}
