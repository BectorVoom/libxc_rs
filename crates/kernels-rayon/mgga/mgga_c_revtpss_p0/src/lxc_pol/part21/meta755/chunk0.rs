//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2647/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2647(t1353: f64, t1883: f64, t46825: f64, t9793: f64, t13848: f64, t9810: f64, t9816: f64, t9818: f64, t1408: f64, t241: f64, t820: f64, t2482: f64, t814: f64, t9991: f64) -> (f64, f64, f64, f64, f64) {
    let t48698 = t1883 * t1353;
    let t48700 = t9793 * t46825 * t48698;
    let t48709 = t9816 * t9818 * t13848 * t9810;
    let t48712 = t820 * t1408 * t241;
    let t48731 = t2482 * t9991 * t814;
    (t48698, t48700, t48709, t48712, t48731)
}
