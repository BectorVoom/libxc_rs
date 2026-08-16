//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 837/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk837(t2385: f64, t315: f64, t323: f64, t157: f64, t2217: f64, t524: f64, t2152: f64, t119: f64, t2387: f64, t310: f64, t557: f64, t8331: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9380 = t315 * t2385;
    let t9381 = t9380 * t323;
    let t9385 = t2217 * t524 * t157;
    let t9386 = t2152 * t9385;
    let t9391 = t119 * t2385;
    let t9397 = t310 * t2387;
    let t9399 = t8331 * t557;
    (t9380, t9381, t9386, t9391, t9397, t9399)
}
