//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1181/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1181(t1692: f64, t637: f64, t1535: f64, t16721: f64, t16775: f64, t16779: f64, t16886: f64, t16889: f64, t16893: f64, t20339: f64, t20341: f64, t20342: f64, t20343: f64, t7201: f64) -> (f64, f64) {
    let t20578 = t637 * t1692;
    let t20586 = 9.0_f64 * t1535 * t1692 * t7201 + t16721 - t16775 - t16779 - t16886 - t16889 - t16893 - t20339 - t20341 - t20342 + t20343;
    (t20578, t20586)
}
