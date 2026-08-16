//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1701/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1701(t1343: f64, t13656: f64, t1448: f64, t198: f64, t3828: f64, t3829: f64, t3889: f64, t39419: f64, t39422: f64, t46280: f64, t46282: f64, t46287: f64, t46290: f64, t46292: f64, t46297: f64, t46298: f64, t46303: f64, t46304: f64, t46345: f64, t5536: f64, t5541: f64, t9547: f64) -> f64 {
    let t46349 = 3.0_f64 * t1343 * t198 * t46345 + 36.0_f64 * t13656 * t198 * t3889 - 4.0_f64 * t1448 * t46304 * t5541 + 18.0_f64 * t198 * t3828 * t46298 + 36.0_f64 * t3829 * t5536 * t9547 - t39419 - t39422 + t46280 + t46282 - t46287 + t46290 - t46292 - t46297 + t46303;
    t46349
}
