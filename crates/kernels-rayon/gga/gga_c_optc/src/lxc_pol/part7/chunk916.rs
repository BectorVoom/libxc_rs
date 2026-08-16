//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 916/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk916(t8627: f64, t8678: f64, t1056: f64, t1037: f64, t8552: f64, t8557: f64, t8560: f64, t8564: f64, t8567: f64, t8571: f64, t8574: f64, t8576: f64, t8579: f64, t8585: f64) -> (f64, f64, f64, f64) {
    let t8679 = t8627 + t8678;
    let t8680 = t8679 * t1056;
    let t8682 = 1.0_f64 * t1037 * t8680;
    let t8683 = t8552 - t8557 - t8560 + t8564 - t8567 + t8571 + t8574 + t8576 + t8579 - t8585 + t8682;
    (t8679, t8680, t8682, t8683)
}
