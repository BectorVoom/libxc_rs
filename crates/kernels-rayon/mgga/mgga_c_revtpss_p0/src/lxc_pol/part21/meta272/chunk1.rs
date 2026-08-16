//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1492/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1492(t4021: f64, t9976: f64, t1398: f64, t1412: f64, t3938: f64, t3992: f64, t2661: f64, t1353: f64, t3889: f64) -> (f64, f64, f64, f64, f64) {
    let t9977 = t9976 * t4021;
    let t9979 = t1412 * t1398;
    let t9980 = t9979 * t3938;
    let t9981 = t3992 * t9980;
    let t9982 = t2661 * t9981;
    let t9984 = t3889 * t1353;
    (t9977, t9979, t9981, t9982, t9984)
}
