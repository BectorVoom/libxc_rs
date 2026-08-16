//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1234/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1234(t13748: f64, t13750: f64, t13754: f64, t13771: f64, t13814: f64, t16233: f64, t16238: f64, t16241: f64, t16244: f64, t16249: f64, t16253: f64, t16255: f64, t16264: f64, t16274: f64) -> f64 {
    let t22590 = -4.0_f64 / 3.0_f64 * t16233 - t16238 / 2.0_f64 + t16241 / 3.0_f64 + t16244 / 6.0_f64 - 5.0_f64 / 3.0_f64 * t16249 + 56.0_f64 / 9.0_f64 * t16253 + 4.0_f64 * t16255 - 4.0_f64 / 3.0_f64 * t16264 - 8.0_f64 / 3.0_f64 * t16274 - t13814 + 140.0_f64 / 27.0_f64 * t13748 + 14.0_f64 / 9.0_f64 * t13750 - 7.0_f64 / 9.0_f64 * t13754 - 5.0_f64 / 3.0_f64 * t13771;
    t22590
}
