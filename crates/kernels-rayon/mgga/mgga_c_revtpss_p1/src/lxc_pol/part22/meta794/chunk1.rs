//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2890/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2890(t3981: f64, t46644: f64, t1412: f64, t3889: f64, t808: f64, t9736: f64, t1369: f64, t9726: f64, t1372: f64, t546: f64, t9801: f64, t9738: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46645 = t46644 * t3981;
    let t46649 = t9736 * t808 * t1412 * t3889;
    let t46651 = t9726 * t1369;
    let t46652 = t46651 * t1372;
    let t46670 = t9801 * t546;
    let t46671 = t46670 * t9738;
    (t46645, t46649, t46651, t46652, t46670, t46671)
}
