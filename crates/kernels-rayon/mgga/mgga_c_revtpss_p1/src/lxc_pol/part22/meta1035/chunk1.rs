//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3621/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3621(t1187: f64, t5180: f64, t16997: f64, t58672: f64, t20567: f64, t300: f64, t1198: f64, t20400: f64, t3539: f64, t5501: f64, t5184: f64, t58665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68605 = t1187 * t5180;
    let t68608 = 0.41016075432865626631e4_f64 * t58672 * t16997 * t68605;
    let t68609 = t300 * t20567;
    let t68611 = 0.11696447245269292414e1_f64 * t68609 * t1198;
    let t68613 = 0.5848223622634646207e0_f64 * t20400 * t3539;
    let t68614 = t5501 * t5501;
    let t68621 = 0.4155806185363551302e3_f64 * t58665 * t5184 * t68605;
    (t68605, t68608, t68611, t68613, t68614, t68621)
}
