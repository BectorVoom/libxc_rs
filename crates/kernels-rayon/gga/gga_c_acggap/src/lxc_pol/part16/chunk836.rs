//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 836/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk836(t1801: f64, t2041: f64, t1805: f64, t1788: f64, t7332: f64, t1809: f64, t570: f64, t1797: f64, t1784: f64, t1886: f64, t2001: f64, t1881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9739 = t2041 * t1801;
    let t9741 = t2041 * t1805;
    let t9743 = t7332 * t1788;
    let t9747 = t570 * t1809;
    let t9749 = t570 * t1797;
    let t9751 = t570 * t1784;
    let t9753 = t2001 * t1886;
    let t9755 = t2001 * t1881;
    (t9739, t9741, t9743, t9747, t9749, t9751, t9753, t9755)
}
