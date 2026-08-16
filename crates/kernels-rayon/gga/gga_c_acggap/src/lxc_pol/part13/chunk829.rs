//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 829/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk829(t150: f64, t187: f64, t8993: f64, t2137: f64, t8396: f64, t2140: f64, t615: f64) -> (f64, f64, f64, f64) {
    let t8995 = t8993 * t150 * t187;
    let t8998 = t2137 * t8396;
    let t8999 = t8998 * t2140;
    let t9003 = t615 * t8396;
    (t8995, t8998, t8999, t9003)
}
