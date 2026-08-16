//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1166/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1166(t16882: f64, t2609: f64, t5342: f64, t16894: f64, t16897: f64, t16721: f64, t16775: f64, t16779: f64, t16886: f64, t16889: f64, t16893: f64, t20329: f64, t20330: f64, t20331: f64, t20333: f64, t20335: f64, t20337: f64, t20338: f64) -> (f64, f64, f64, f64, f64) {
    let t20339 = 96.0_f64 * t16882;
    let t20340 = t2609 * t5342;
    let t20341 = 0.5848223622634646207e0_f64 * t20340;
    let t20342 = 4.0_f64 * t16894;
    let t20343 = 3.0_f64 * t16897;
    let t20344 = t20329 - t20330 - t20331 + t20333 - t20335 + t20337 + t20338 - t20339 - t16886 - t16889 - t20341 - t16893 - t20342 + t20343 + t16721 - t16775 - t16779;
    (t20339, t20341, t20342, t20343, t20344)
}
