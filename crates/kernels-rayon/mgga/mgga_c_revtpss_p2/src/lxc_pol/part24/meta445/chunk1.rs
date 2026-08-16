//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1406/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1406(t1892: f64, t5744: f64, t786: f64, t1320: f64, t13632: f64, t1317: f64, t3857: f64, t5569: f64, t1856: f64, t512: f64, t9544: f64, t5571: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48083 = t5744 * t1892;
    let t48084 = t786 * t48083;
    let t48152 = t1320 * t13632;
    let t48225 = t1317 * t13632;
    let t48227 = t3857 * t5569;
    let t48243 = t512 * t1856 * t9544;
    let t48262 = t5571 * t9387;
    (t48084, t48152, t48225, t48227, t48243, t48262)
}
