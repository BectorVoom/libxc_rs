//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1717/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1717(t1353: f64, t13767: f64, t2661: f64, t3889: f64, t4010: f64, t240: f64, t9991: f64, t550: f64, t9898: f64, t9994: f64, t3992: f64, t543: f64, t9890: f64) -> (f64, f64, f64, f64) {
    let t46607 = t2661 * t13767 * t4010 * t3889 * t1353;
    let t46609 = t9991 * t240;
    let t46610 = t550 * t9898;
    let t46613 = t2661 * t46609 * t46610 * t9994;
    let t46618 = t2661 * t3992 * t550 * t9890 * t543;
    (t46607, t46610, t46613, t46618)
}
