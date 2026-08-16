//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1720/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1720(t125: f64, t9898: f64, t13999: f64, t9837: f64, t546: f64, t9801: f64, t9738: f64, t124: f64, t3938: f64, t4056: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64) {
    let t46655 = t125 * t9898;
    let t46660 = t13999 * t9837;
    let t46670 = t9801 * t546;
    let t46671 = t46670 * t9738;
    let t46680 = t9816 * t9818 * t124 * t4056 * t3938;
    (t46655, t46660, t46671, t46680)
}
