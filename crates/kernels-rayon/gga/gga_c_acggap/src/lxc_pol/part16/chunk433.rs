//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 433/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk433(t372: f64, t604: f64, t142: f64, t2060: f64, t592: f64, t595: f64) -> (f64, f64, f64, f64) {
    let t2061 = t604 * t372;
    let t2062 = t142 * t2061;
    let t2063 = t2060 * t2062;
    let t2065 = t592 * t595;
    (t2061, t2062, t2063, t2065)
}
