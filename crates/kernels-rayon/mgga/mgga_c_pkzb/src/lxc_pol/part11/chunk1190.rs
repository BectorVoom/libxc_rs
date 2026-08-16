//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1190/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1190(t20334: f64, t24642: f64, t20340: f64, t16721: f64, t16775: f64, t16779: f64, t16783: f64, t16787: f64, t16875: f64, t16886: f64, t16889: f64, t16893: f64, t16897: f64, t19825: f64, t20337: f64, t20338: f64, t20339: f64) -> (f64, f64, f64, f64) {
    let t29126 = 36.0_f64 * t20334;
    let t29127 = 0.73245789224026180216e-3_f64 * t24642;
    let t29128 = 0.17544670867903938621e1_f64 * t20340;
    let t29129 = -t19825 - t16875 - t29126 - t20337 - t20338 + t20339 + t29127 - t16886 - t16889 - t29128 - t16893 + t16897 + t16721 - t16775 - t16779 + t16783 - t16787;
    (t29126, t29127, t29128, t29129)
}
