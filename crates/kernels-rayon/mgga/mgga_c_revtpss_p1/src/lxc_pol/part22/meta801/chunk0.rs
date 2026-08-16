//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2902/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2902(t4086: f64, t9801: f64, t9846: f64, t3889: f64, t4003: f64, t3855: f64, t3860: f64, t1320: f64, t9545: f64, t3863: f64, t3857: f64, t40082: f64, t512: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    let t46951 = t4003 * t3889;
    let t46960 = t3860 * t3855;
    let t46963 = 16.0_f64 * t1320 * t9545;
    let t46964 = t3863 * t3855;
    let t46967 = t3857 * t3855;
    let t46970 = t512 * t520 * t40082;
    (t46946, t46947, t46951, t46960, t46963, t46964, t46967, t46970)
}
