//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 281/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk281(t157: f64, t360: f64, t372: f64, t119: f64, t441: f64, t186: f64, t447: f64) -> (f64, f64, f64, f64) {
    let t1182 = t157 * t360;
    let t1188 = t157 * t372;
    let t1215 = t119 * t441;
    let t1219 = 1.0_f64 / t447 / t186;
    (t1182, t1188, t1215, t1219)
}
