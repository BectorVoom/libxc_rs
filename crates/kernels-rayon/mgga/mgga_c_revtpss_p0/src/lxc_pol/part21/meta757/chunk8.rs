//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2663/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2663(t1413: f64, t46835: f64, t48698: f64, t13845: f64, t13847: f64, t13848: f64, t4004: f64, t1872: f64, t9818: f64, t1873: f64, t46651: f64, t1399: f64, t5689: f64, t9816: f64) -> (f64, f64, f64, f64, f64) {
    let t49012 = t46835 * t1413 * t48698;
    let t49016 = t13845 * t13847 * t13848 * t4004;
    let t49024 = t13845 * t9818 * t1872 * t4004;
    let t49030 = t46651 * t1873;
    let t49049 = t9816 * t9818 * t5689 * t1399;
    (t49012, t49016, t49024, t49030, t49049)
}
