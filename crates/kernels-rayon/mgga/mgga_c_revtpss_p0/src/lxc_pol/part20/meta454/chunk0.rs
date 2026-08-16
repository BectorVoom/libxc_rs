//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1737/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1737(t9744: f64, t9966: f64, t3889: f64, t4003: f64, t3855: f64, t3860: f64, t1320: f64, t9545: f64, t3863: f64, t39419: f64, t39422: f64, t46280: f64, t46282: f64, t46287: f64, t46290: f64, t46292: f64, t46297: f64, t46303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46949 = t9744 * t9966;
    let t46951 = t4003 * t3889;
    let t46960 = t3860 * t3855;
    let t46961 = 72.0_f64 * t46960;
    let t46963 = 16.0_f64 * t1320 * t9545;
    let t46964 = t3863 * t3855;
    let t46965 = 192.0_f64 * t46964;
    let t46966 = t46280 + t46282 - t46287 + t46290 - t46292 - t46297 - t39419 - t39422 + t46303 + t46961 - t46963 - t46965;
    (t46949, t46951, t46961, t46963, t46965, t46966)
}
