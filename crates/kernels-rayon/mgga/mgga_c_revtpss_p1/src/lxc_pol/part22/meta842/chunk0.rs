//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2974/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2974(t14230: f64, t46802: f64, t49068: f64, t46888: f64, t48908: f64, t1398: f64, t5591: f64, t13946: f64, t9962: f64, t1413: f64, t46835: f64, t48694: f64) -> (f64, f64, f64, f64, f64) {
    let t49103 = t46802 * t49068 * t14230;
    let t49105 = t46888 * t48908;
    let t49107 = t5591 * t1398;
    let t49118 = t9962 * t13946;
    let t49121 = t46835 * t1413 * t48694;
    (t49103, t49105, t49107, t49118, t49121)
}
